use anyhow::Result;
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::{map::Entry, IndexMap, IndexSet};
use itertools::Itertools;

use crate::{bls12_377::Field, r1cs::*};

#[derive(Debug, Clone)]
enum NamedObject {
    Constraint(usize),
    Var(Variable),
    // contains the list of named objects that belong to it
    Namespace(Namespace),
}

#[derive(Debug, Clone, Default)]
struct Namespace {
    children: Vec<NamedObject>,
    idx: NamespaceIndex,
}

impl Namespace {
    fn push(&mut self, child: NamedObject) {
        self.children.push(child);
    }
}

type InternedField = usize;
type InternedPathSegment = usize;
type NamespaceIndex = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InternedPath {
    parent_namespace: NamespaceIndex,
    last_segment: InternedPathSegment,
}

#[derive(PartialEq, Eq, Hash)]
pub struct TestConstraint {
    interned_path: InternedPath,
    a: Vec<(Variable, InternedField)>,
    b: Vec<(Variable, InternedField)>,
    c: Vec<(Variable, InternedField)>,
}

#[derive(Default, Debug)]
pub struct CurrentNamespace {
    segments: Vec<InternedPathSegment>,
    indices: Vec<NamespaceIndex>,
}

impl CurrentNamespace {
    fn idx(&self) -> usize {
        self.indices.last().copied().unwrap_or(0)
    }

    fn pop(&mut self) {
        assert!(self.segments.pop().is_some());
        assert!(self.indices.pop().is_some());
    }
}

/// Constraint system for testing purposes.
pub struct TestConstraintSystem<F: Field> {
    // used to intern full paths in test scenarios, for get and set purposes
    interned_full_paths: FxHashMap<Vec<InternedPathSegment>, InternedPath>,
    // used to intern namespace segments
    interned_path_segments: IndexSet<String, FxBuildHasher>,
    // used to intern fields belonging to F
    interned_fields: IndexSet<F, FxBuildHasher>,
    // contains named objects bound to their (interned) paths; the indices are
    // used for NamespaceIndex lookups
    named_objects: IndexMap<InternedPath, NamedObject, FxBuildHasher>,
    // a stack of current path's segments and the index of the current path's
    // index in the named_objects map
    current_namespace: CurrentNamespace,
    // the list of currently applicable constraints
    constraints: OptionalVec<TestConstraint>,
    // the list of currently applicable input variables
    public_variables: OptionalVec<InternedField>,
    // the list of currently applicable auxiliary variables
    private_variables: OptionalVec<InternedField>,
}

impl<F: Field> Default for TestConstraintSystem<F> {
    fn default() -> Self {
        let mut interned_path_segments = IndexSet::with_hasher(FxBuildHasher::default());
        let path_segment = "ONE".to_owned();
        let interned_path_segment = interned_path_segments.insert_full(path_segment).0;
        let interned_path = InternedPath {
            parent_namespace: 0,
            last_segment: interned_path_segment,
        };

        let mut interned_full_paths = FxHashMap::default();
        interned_full_paths.insert(vec![interned_path_segment], interned_path);

        let mut named_objects = IndexMap::with_hasher(FxBuildHasher::default());
        named_objects.insert_full(
            interned_path,
            NamedObject::Var(TestConstraintSystem::<F>::one()),
        );

        let mut interned_fields = IndexSet::with_hasher(FxBuildHasher::default());
        let interned_field = interned_fields.insert_full(F::ONE).0;

        let mut inputs: OptionalVec<InternedField> = Default::default();
        inputs.insert(interned_field);

        let constraints = OptionalVec::default();

        TestConstraintSystem {
            interned_full_paths,
            interned_fields,
            interned_path_segments,
            named_objects,
            current_namespace: Default::default(),
            constraints,
            public_variables: inputs,
            private_variables: Default::default(),
        }
    }
}

impl<F: Field> TestConstraintSystem<F> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Prints the constraint at which `self` and `other` differ.
    pub fn diff(&self, other: &Self) {
        for (i, (self_c, other_c)) in self
            .constraints
            .iter()
            .zip(other.constraints.iter())
            .enumerate()
        {
            let self_interned_path = self_c.interned_path;
            let other_interned_path = other_c.interned_path;
            if self_c.a != other_c.a {
                println!("A row {} is different:", i);
                println!("self: {}", self.unintern_path(self_interned_path));
                println!("other: {}", other.unintern_path(other_interned_path));
                break;
            }

            if self_c.b != other_c.b {
                println!("B row {} is different:", i);
                println!("self: {}", self.unintern_path(self_interned_path));
                println!("other: {}", other.unintern_path(other_interned_path));
                break;
            }

            if self_c.c != other_c.c {
                println!("C row {} is different:", i);
                println!("self: {}", self.unintern_path(self_interned_path));
                println!("other: {}", other.unintern_path(other_interned_path));
                break;
            }
        }
    }

    fn intern_path(&self, path: &str) -> InternedPath {
        let mut vec = vec![];

        for segment in path.split('/') {
            vec.push(self.interned_path_segments.get_index_of(segment).unwrap());
        }

        *self.interned_full_paths.get(&vec).unwrap()
    }

    fn unintern_path(&self, interned_path: InternedPath) -> String {
        let last_segment = self
            .interned_path_segments
            .get_index(interned_path.last_segment)
            .unwrap();
        let mut reversed_uninterned_segments = vec![last_segment];

        let mut parent_ns = interned_path.parent_namespace;
        while parent_ns != 0 {
            let interned_parent_ns = self.named_objects.get_index(parent_ns).unwrap().0;
            let parent_segment = self
                .interned_path_segments
                .get_index(interned_parent_ns.last_segment)
                .unwrap();
            reversed_uninterned_segments.push(parent_segment);
            parent_ns = interned_parent_ns.parent_namespace;
        }

        let segments = reversed_uninterned_segments
            .into_iter()
            .map(|s| s.as_str())
            .rev();
        Itertools::intersperse(segments, "/").collect()
    }

    pub fn print_named_objects(&self) {
        for TestConstraint { interned_path, .. } in self.constraints.iter() {
            println!("{}", self.unintern_path(*interned_path));
        }
    }

    fn eval_lc(&self, terms: &[(Variable, InternedField)]) -> F {
        let mut acc = F::ZERO;

        for &(var, interned_coeff) in terms {
            let interned_tmp = match var.get_unchecked() {
                Index::Public(index) => self.public_variables[index],
                Index::Private(index) => self.private_variables[index],
            };
            let mut tmp = *self.interned_fields.get_index(interned_tmp).unwrap();
            let coeff = self.interned_fields.get_index(interned_coeff).unwrap();

            tmp.mul_assign(coeff);
            acc.add_assign(tmp);
        }

        acc
    }

    pub fn which_is_unsatisfied(&self) -> Option<String> {
        for TestConstraint {
            interned_path,
            a,
            b,
            c,
        } in self.constraints.iter()
        {
            let mut a = self.eval_lc(a.as_ref());
            let b = self.eval_lc(b.as_ref());
            let c = self.eval_lc(c.as_ref());

            a.mul_assign(&b);

            if a != c {
                return Some(self.unintern_path(*interned_path));
            }
        }

        None
    }

    pub fn is_satisfied(&self) -> bool {
        self.which_is_unsatisfied().is_none()
    }

    pub fn num_non_zero(&self) -> (usize, usize, usize) {
        let mut non_zero_a = 0;
        let mut non_zero_b = 0;
        let mut non_zero_c = 0;
        for TestConstraint { a, b, c, .. } in self.constraints.iter() {
            non_zero_a += a.len();
            non_zero_b += b.len();
            non_zero_c += c.len();
        }
        (non_zero_a, non_zero_b, non_zero_c)
    }

    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    pub fn get_constraint_path(&self, i: usize) -> String {
        self.unintern_path(self.constraints.iter().nth(i).unwrap().interned_path)
    }

    pub fn set(&mut self, path: &str, to: F) {
        let interned_path = self.intern_path(path);
        let interned_field = self.interned_fields.insert_full(to).0;

        match self.named_objects.get(&interned_path) {
            Some(&NamedObject::Var(ref v)) => match v.get_unchecked() {
                Index::Public(index) => self.public_variables[index] = interned_field,
                Index::Private(index) => self.private_variables[index] = interned_field,
            },
            Some(e) => panic!(
                "tried to set path `{}` to value, but `{:?}` already exists there.",
                path, e
            ),
            _ => panic!("no variable exists at path: {}", path),
        }
    }

    pub fn get(&mut self, path: &str) -> F {
        let interned_path = self.intern_path(path);

        let interned_field = match self.named_objects.get(&interned_path) {
            Some(&NamedObject::Var(ref v)) => match v.get_unchecked() {
                Index::Public(index) => self.public_variables[index],
                Index::Private(index) => self.private_variables[index],
            },
            Some(e) => panic!(
                "tried to get value of path `{}`, but `{:?}` exists there (not a variable)",
                path, e
            ),
            _ => panic!("no variable exists at path: {}", path),
        };

        *self.interned_fields.get_index(interned_field).unwrap()
    }

    fn set_named_obj(&mut self, interned_path: InternedPath, to: NamedObject) -> NamespaceIndex {
        match self.named_objects.entry(interned_path) {
            Entry::Vacant(e) => {
                let ns_idx = e.index();
                e.insert(to);
                ns_idx
            }
            Entry::Occupied(e) => {
                let interned_segments = e.remove_entry().0;
                panic!(
                    "tried to create object at existing path: {}",
                    self.unintern_path(interned_segments)
                );
            }
        }
    }

    fn compute_path(&mut self, new_segment: &str) -> InternedPath {
        let (interned_segment, new) =
            if let Some(index) = self.interned_path_segments.get_index_of(new_segment) {
                (index, false)
            } else {
                self.interned_path_segments
                    .insert_full(new_segment.to_owned())
            };

        // only perform the check for segments not seen before
        assert!(
            !new || !new_segment.contains('/'),
            "'/' is not allowed in names"
        );

        let interned_path = InternedPath {
            parent_namespace: self.current_namespace.idx(),
            last_segment: interned_segment,
        };

        let mut full_path = self.current_namespace.segments.clone();
        full_path.push(interned_segment);
        self.interned_full_paths.insert(full_path, interned_path);

        interned_path
    }

    fn register_object_in_namespace(&mut self, named_obj: NamedObject) {
        if let NamedObject::Namespace(ref mut ns) = self
            .named_objects
            .get_index_mut(self.current_namespace.idx())
            .unwrap()
            .1
        {
            ns.push(named_obj);
        }
    }
}

impl<F: Field> ConstraintSystem for TestConstraintSystem<F> {
    type Root = Self;

    fn alloc<Fn, A, F, AR>(&mut self, annotation: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<F>,
        A: FnOnce() -> AR,
        F: Field,
        AR: AsRef<str>,
    {
        let interned_path = self.compute_path(annotation().as_ref());
        let interned_field = self.interned_fields.insert_full(f()?).0;
        let index = self.private_variables.insert(interned_field);
        let var = Variable::new_unchecked(Index::Private(index));
        let named_obj = NamedObject::Var(var);
        self.register_object_in_namespace(named_obj.clone());
        self.set_named_obj(interned_path, named_obj);

        Ok(var)
    }

    fn alloc_input<Fn, A, F, AR>(&mut self, annotation: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<F>,
        A: FnOnce() -> AR,
        F: Field,
        AR: AsRef<str>,
    {
        let interned_path = self.compute_path(annotation().as_ref());
        let interned_field = self.interned_fields.insert_full(f()?).0;
        let index = self.public_variables.insert(interned_field);
        let var = Variable::new_unchecked(Index::Public(index));
        let named_obj = NamedObject::Var(var);
        self.register_object_in_namespace(named_obj.clone());
        self.set_named_obj(interned_path, named_obj);

        Ok(var)
    }

    fn enforce<A, F, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        F: Field,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        let interned_path = self.compute_path(annotation().as_ref());
        let index = self.constraints.next_idx();
        let named_obj = NamedObject::Constraint(index);
        self.register_object_in_namespace(named_obj.clone());
        self.set_named_obj(interned_path, named_obj);

        let mut intern_fields = |uninterned: Vec<(Variable, F)>| -> Vec<(Variable, InternedField)> {
            uninterned
                .into_iter()
                .map(|(var, field)| {
                    let interned_field = self.interned_fields.insert_full(field).0;
                    (var, interned_field)
                })
                .collect()
        };

        let a = intern_fields(a(LinearCombination::zero()).0);
        let b = intern_fields(b(LinearCombination::zero()).0);
        let c = intern_fields(c(LinearCombination::zero()).0);

        self.constraints.insert(TestConstraint {
            interned_path,
            a,
            b,
            c,
        });
    }

    fn push_namespace<NR: AsRef<str>, N: FnOnce() -> NR>(&mut self, name_fn: N) {
        let name = name_fn();
        let interned_path = self.compute_path(name.as_ref());
        let new_segment = interned_path.last_segment;
        let named_obj = NamedObject::Namespace(Default::default());
        self.register_object_in_namespace(named_obj.clone());
        let namespace_idx = self.set_named_obj(interned_path, named_obj);
        if let NamedObject::Namespace(ref mut ns) = self.named_objects[namespace_idx] {
            ns.idx = namespace_idx;
        };

        self.current_namespace.segments.push(new_segment);
        self.current_namespace.indices.push(namespace_idx);
    }

    fn pop_namespace(&mut self) {
        // don't perform a full cleanup in test conditions, so that all the variables,
        // constraints and namespace indices remain available throughout the tests

        self.current_namespace.pop();
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn num_constraints(&self) -> usize {
        self.num_constraints()
    }

    fn num_public_variables(&self) -> usize {
        self.public_variables.len()
    }

    fn num_private_variables(&self) -> usize {
        self.private_variables.len()
    }

    fn is_in_setup_mode(&self) -> bool {
        false
    }
}
