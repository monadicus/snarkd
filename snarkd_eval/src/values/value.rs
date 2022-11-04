use anyhow::{bail, Result};

use crate::ConstraintSystem;

use super::*;

#[derive(Clone, Debug)]
pub enum ConstrainedValue<F: Field, G: Group> {
    Address(ConstrainedAddress<G>),
    Boolean(ConstrainedBool),
    Field(ConstrainedField<F>),
    Group(ConstrainedGroup<G>),
    Integer(ConstrainedInteger),
    Scalar(ConstrainedScalar),
    String(ConstrainedString),
    Struct(ConstrainedStructure<F, G>),
    Record(ConstrainedRecord<F, G>),
}

impl<F: Field, G: Group> ConstrainedValue<F, G> {
    pub fn extract_bool(&self) -> Result<&ConstrainedBool, &Self> {
        match self {
            ConstrainedValue::Boolean(x) => Ok(x),
            value => Err(value),
        }
    }

    pub fn extract_integer(&self) -> Result<&ConstrainedInteger, &Self> {
        match self {
            ConstrainedValue::Integer(x) => Ok(x),
            value => Err(value),
        }
    }

    pub fn matches_input_type(&self, type_: &Type) -> bool {
        match (self, type_) {
            (ConstrainedValue::Address(_), Type::Address)
            | (ConstrainedValue::Boolean(_), Type::Boolean)
            | (ConstrainedValue::Field(_), Type::Field)
            | (ConstrainedValue::Group(_), Type::Group)
            | (ConstrainedValue::Integer(ConstrainedInteger::I8(_)), Type::I8)
            | (ConstrainedValue::Integer(ConstrainedInteger::I16(_)), Type::I16)
            | (ConstrainedValue::Integer(ConstrainedInteger::I32(_)), Type::I32)
            | (ConstrainedValue::Integer(ConstrainedInteger::I64(_)), Type::I64)
            | (ConstrainedValue::Integer(ConstrainedInteger::I128(_)), Type::I128)
            | (ConstrainedValue::Integer(ConstrainedInteger::U8(_)), Type::U8)
            | (ConstrainedValue::Integer(ConstrainedInteger::U16(_)), Type::U16)
            | (ConstrainedValue::Integer(ConstrainedInteger::U32(_)), Type::U32)
            | (ConstrainedValue::Integer(ConstrainedInteger::U64(_)), Type::U64)
            | (ConstrainedValue::Integer(ConstrainedInteger::U128(_)), Type::U128) => true,
            (_, _) => false,
        }
    }
}

impl<F: Field, G: Group> ConstrainedValue<F, G> {
    pub fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &ConstrainedBool,
        first: &Self,
        second: &Self,
    ) -> Result<Self> {
        Ok(match (first, second) {
            (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
                ConstrainedValue::Address(ConstrainedAddress::conditionally_select(
                    cs, cond, address_1, address_2,
                )?)
            }
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                ConstrainedValue::Boolean(ConstrainedBool::conditionally_select(
                    cs, cond, bool_1, bool_2,
                )?)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                ConstrainedValue::Field(ConstrainedField::conditionally_select(
                    cs, cond, field_1, field_2,
                )?)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                ConstrainedValue::Group(todo!())
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                ConstrainedValue::Integer(ConstrainedInteger::conditionally_select(
                    cs, cond, num_1, num_2,
                )?)
            }
            (_, _) => bail!("Unsatisfiable"),
        })
    }
}
