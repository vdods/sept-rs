use crate::{
    dy, parser, qv,
    st::{self, StringifiableT, TermT},
    Error, Result,
};

pub trait FancyAnyT: std::any::Any {
    fn as_any<'a>(&'a self) -> &'a (dyn std::any::Any + Send + Sync);
    fn as_any_mut<'a>(&'a mut self) -> &'a mut (dyn std::any::Any + Send + Sync);
    fn as_fancy_any<'a>(&'a self) -> &'a (dyn FancyAnyT + Send + Sync);
    // // TODO
    // // fn type_name(&self) -> &'static str;
}

impl<T: std::any::Any + Send + Sync> FancyAnyT for T {
    fn as_any<'a>(&'a self) -> &'a (dyn std::any::Any + Send + Sync) {
        self
    }
    fn as_any_mut<'a>(&'a mut self) -> &'a mut (dyn std::any::Any + Send + Sync) {
        self
    }
    fn as_fancy_any<'a>(&'a self) -> &'a (dyn FancyAnyT + Send + Sync) {
        self
    }
}

pub type ValueGuts = dyn std::any::Any + Send + Sync;
// pub type ValueGuts = dyn FancyAnyT + Send + Sync;
pub type ValueGuts2 = dyn FancyAnyT + Send + Sync;

impl ValueGuts2 {
    pub fn is<T: std::any::Any>(&self) -> bool {
        // <dyn std::any::Any + Send + Sync>::is::<T>(self)
        self.as_any().is::<T>()
    }
    // pub fn downcast<T: std::any::Any, A: Allocator>(self: Box<ValueGuts>) -> Result<Box<T, A>> {
    //     unimplemented!("blah");
    // }
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        // std::any::Any::downcast_ref(self)
        self.as_any().downcast_ref::<T>()
    }
    pub fn downcast_mut<T: std::any::Any>(&mut self) -> Option<&mut T> {
        // std::any::Any::downcast_mut(self)
        self.as_any_mut().downcast_mut::<T>()
    }
}

// impl<A: Allocator> Box<dyn FancyAnyT + Send + Sync, A> {
//     pub fn downcast<T: std::any::Any>(self) -> Result<Box<T, A>, Self> {
//         unimplemented!("blah");
//     }
// }

impl std::fmt::Debug for ValueGuts2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueGuts(")?;
        dy::RUNTIME_LA.read().unwrap().debug(self.as_any(), f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl ToOwned for ValueGuts2 {
    type Owned = Value;
    fn to_owned(&self) -> Self::Owned {
        panic!("temp hack");
        // Value(dy::RUNTIME_LA.read().unwrap().clone(self.as_any()))
    }
    fn clone_into(&self, _target: &mut Self::Owned) {
        panic!("temp hack");
        // target.0 = dy::RUNTIME_LA.read().unwrap().clone(self.as_any());
    }
}

impl PartialEq for ValueGuts2 {
    fn eq(&self, other: &Self) -> bool {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .eq(self.as_any(), other.as_any())
    }
}

impl Eq for ValueGuts2 {}

impl PartialOrd for ValueGuts2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .partial_cmp(self.as_any(), other.as_any())
    }
}

impl Ord for ValueGuts2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .cmp(self.as_any(), other.as_any())
    }
}

// // TEMP EXPERIMENTAL
// #[derive(derive_more::Deref)]
// pub struct ValueGutsRef<'a>(&'a ValueGuts);

// pub struct ValueGutsThingy(ValueGuts);

// // TEMP EXPERIMENTAL
// impl std::borrow::ToOwned for ValueGutsThingy {
//     type Owned = Value;
//     fn to_owned(&self) -> Self::Owned {
//         Value(dy::RUNTIME_LA.read().unwrap().clone(&self.0))
//     }
//     fn clone_into(&self, target: &mut Self::Owned) {
//         target.0 = dy::RUNTIME_LA.read().unwrap().clone(&self.0);
//     }
// }

// // TEMP EXPERIMENTAL
// impl<T: dy::IntoValueT> std::borrow::ToOwned for T {
//     type Owned = Value;
//     fn to_owned(&self) -> Self::Owned {
//         Value(dy::RUNTIME_LA.read().unwrap().clone(self))
//     }
//     fn clone_into(&self, target: &mut Self::Owned) {
//         target.0 = dy::RUNTIME_LA.read().unwrap().clone(self);
//     }
// }

// // TEMP EXPERIMENTAL
// impl<'a> std::borrow::ToOwned for ValueGutsRef<'a> {
//     type Owned = Value;
//     fn to_owned(&self) -> Self::Owned {
//         Value(dy::RUNTIME_LA.read().unwrap().clone(self))
//     }
//     fn clone_into(&self, target: &mut Self::Owned) {
//         target.0 = dy::RUNTIME_LA.read().unwrap().clone(self);
//     }
// }

// // TEMP EXPERIMENTAL
// impl<'a> std::borrow::Borrow<ValueGutsRef<'a>> for Value {
//     fn borrow(&self) -> &ValueGutsRef<'a> {
//         ValueGutsRef(self.0.borrow())
//     }
// }

// TEMP EXPERIMENTAL
impl std::borrow::Borrow<ValueGuts2> for Value {
    fn borrow(&self) -> &ValueGuts2 {
        self.0.as_fancy_any()
    }
}

/// This is the central runtime data type for sept.  Its methods will call into the corresponding
/// methods of the runtime.
// This really should be named Term (or more pedantically TermTerm), but that's already taken.
// Maybe the naming scheme can be shifted around so that this can be named Term (or TermTerm).
#[derive(derive_more::Into)]
pub struct Value(Box<ValueGuts>);

impl qv::ApplyEditT for Value {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // Handle some of the canonical edits first, and then resort to the Runtime.
        if edit.is::<st::NoOp>() {
            // Nothing to do.
            Ok(())
        } else if edit.is::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            anyhow::ensure!(
                *self == replacement_term.old_data,
                "ReplacementTerm edit expected current value to match old_data"
            );
            *self = replacement_term.new_data;
            Ok(())
        } else {
            dy::RUNTIME_LA
                .read()
                .unwrap()
                .apply_edit(self.as_mut(), edit)
        }
    }
}

impl AsMut<ValueGuts> for Value {
    fn as_mut(&mut self) -> &mut ValueGuts {
        self.0.as_mut()
    }
}

impl AsRef<ValueGuts> for Value {
    fn as_ref(&self) -> &ValueGuts {
        self.0.as_ref()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        // Value(dy::RUNTIME_LA.read().unwrap().clone(self.as_ref()))
        Value::from(dy::RUNTIME_LA.read().unwrap().clone(self.as_ref()))
    }
}

impl dy::ConstructorT for Value {
    type ConstructedType = Value;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        Ok(dy::RUNTIME_LA
            .read()
            .unwrap()
            .construct(self.as_ref(), parameter_t)?)
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        Ok(dy::RUNTIME_LA
            .read()
            .unwrap()
            .deserialize_parameters_and_construct(self.as_ref(), reader)?)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Value(")?;
        dy::RUNTIME_LA.read().unwrap().debug(self.as_ref(), f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl dy::DeconstructT for Value {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Implement self-consuming deconstruct in Runtime.
        dy::RUNTIME_LA.read().unwrap().deconstructed(self.as_ref())
    }
    fn deconstructed(&self) -> dy::Deconstruction {
        dy::RUNTIME_LA.read().unwrap().deconstructed(self.as_ref())
    }
}

impl std::ops::Deref for Value {
    type Target = ValueGuts;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

// impl std::fmt::Debug for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "Value({})", &self.stringify())
//     }
// }

impl st::DeserializableT for Value {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        // First read the SerializedTopLevelCode to decide what to do.
        match st::SerializedTopLevelCode::read(reader)? {
            st::SerializedTopLevelCode::Construction => {
                // Deserialize the constructor.
                let constructor = Value::deserialize(reader)?;
                // Deserialize the parameters and construct the Value.
                use dy::ConstructorT;
                Ok(constructor.deserialize_parameters_and_construct(reader)?)
            }
            st::SerializedTopLevelCode::NonParametric => {
                // The NonParametricTermCode plays the role of the constructor, and there are
                // no parameters as you might have guessed.
                let non_parametric_term_code = st::NonParametricTermCode::read(reader)?;
                Ok(dy::RUNTIME_LA
                    .read()
                    .unwrap()
                    .non_parametric_term_from_code(non_parametric_term_code)?)
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl st::EditT for Value {
    type Inverse = Self;
    fn into_inverse(self) -> Self::Inverse {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .into_inverse(self)
            .expect("into_inverse not registered for this type in the Runtime.")
    }
}

impl Eq for Value {}

/// This prevents directly nested Value-s, e.g. Value(Value(123u32)), since that's never what we want.
impl From<Box<ValueGuts>> for Value {
    fn from(b: Box<ValueGuts>) -> Self {
        if b.is::<Value>() {
            log::warn!("Some code attempted to construct a directly nested Value (i.e. `Value(Value(...))`).  Preventing that by unwrapping all but the inner Value, since directly nested Value-s are never what we want.");
            Self::from(b.downcast::<Value>().unwrap().into_inner())
        } else {
            Self(b)
        }
    }
}

impl<T: TermT + dy::IntoValueT + 'static> From<T> for Value {
    fn from(t: T) -> Self {
        Self(Box::new(t))
    }
}

impl std::str::FromStr for Value {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parser::parse_value(s)?)
    }
}

impl st::InhabitsT<Value> for Value {
    fn inhabits(&self, rhs: &Value) -> bool {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .inhabits(self.as_ref(), rhs.as_ref())
    }
}

impl<T: st::TypeT + dy::IntoValueT + 'static> st::InhabitsT<T> for Value {
    fn inhabits(&self, rhs: &T) -> bool {
        let rhs_: &ValueGuts = rhs;
        dy::RUNTIME_LA.read().unwrap().inhabits(self.as_ref(), rhs_)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .cmp(self.as_ref(), other.as_ref())
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .eq(self.as_ref(), other.as_ref())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .partial_cmp(self.as_ref(), other.as_ref())
    }
}

// impl qv::EvalT for Value {
//     fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
//         Ok(dy::MaybeDereferencedValue::make_ref(self.as_ref()))
//     }
// }

impl qv::QueryableDynT for Value {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::ValueView::new(self))
    }
}

impl qv::QueryMutAndApplyEditT for Value {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // Have to do some extra handling of if the address iterator is empty.
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            // We have to shunt it to Value::apply_edit because that does special
            // handling before forwarding it to the Runtime.
            use qv::ApplyEditT;
            self.apply_edit(edit)
        } else {
            dy::RUNTIME_LA.read().unwrap().query_mut_and_apply_edit(
                self.as_mut(),
                &mut address_token_i,
                edit,
            )
        }
    }
}

impl st::SerializableT for Value {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        log::trace!("Value::serialize; self: {:?}", self);
        let mut bytes_written = 0usize;
        if self.nondereferencing_is_parametric() {
            let constructor = self.nondereferencing_abstract_type();
            log::trace!("    constructor: {:?}", constructor);
            // This indicates this serialization is a construction.
            bytes_written += st::SerializedTopLevelCode::Construction.write(writer)?;
            // This is the constructor
            bytes_written += constructor.serialize(writer)?;
            // This is the value itself.  Note that Runtime::serialize is a non-dereferencing serialize.
            bytes_written += dy::RUNTIME_LA
                .read()
                .unwrap()
                .serialize(self.as_ref(), writer)?;
        } else {
            // This indicates this serialization is a non-parametric term.
            bytes_written += st::SerializedTopLevelCode::NonParametric.write(writer)?;
            // This is the code representing the non-parametric term itself (its "value").
            bytes_written += dy::RUNTIME_LA
                .read()
                .unwrap()
                .non_parametric_term_code(self.as_ref())?
                .write(writer)?;
        }
        Ok(bytes_written)

        //         // Because Value stores type information dynamically, it all has to be serialized.
        //         let mut bytes_written =
        //             dy::RUNTIME_LA.read().unwrap().serialize_top_level_code(self.as_ref(), writer)?;
        //         // TODO: This would be replaced with getting the constructor (assuming the value is
        //         // Constructible) and then serializing that.
        //         bytes_written +=
        //             dy::RUNTIME_LA.read().unwrap().serialize_constructor(self.as_ref(), writer)?;
        //         bytes_written +=
        //             dy::RUNTIME_LA.read().unwrap().serialize_parameters(self.as_ref(), writer)?;
        //         Ok(bytes_written)
    }

    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_top_level_code(self.as_ref(), writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_constructor(self.as_ref(), writer)?)
    //     }
    //     fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_parameters(self.as_ref(), writer)?)
    //     }
}

impl StringifiableT for Value {
    fn stringify(&self) -> String {
        dy::RUNTIME_LA.read().unwrap().stringify(self.as_ref())
    }
}

impl TermT for Value {
    type AbstractTypeType = Value;

    fn is_parametric(&self) -> bool {
        dy::RUNTIME_LA.read().unwrap().is_parametric(self.as_ref())
    }
    fn is_type(&self) -> bool {
        dy::RUNTIME_LA.read().unwrap().is_type(self.as_ref())
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Value::from(
            dy::RUNTIME_LA
                .read()
                .unwrap()
                .abstract_type_of(self.as_ref()),
        )
        // Value(
        //     dy::RUNTIME_LA
        //         .read()
        //         .unwrap()
        //         .abstract_type_of(self.as_ref()),
        // )
    }
}

impl st::TypeT for Value {}

// TODO: These could become part of dy::TermT, since they reflect what's available via Runtime
impl Value {
    pub fn into_inner(self) -> Box<ValueGuts> {
        self.0
    }
    /// This will return the downcasted value, consuming self, or panic if the cast fails.
    // pub fn downcast_into<T: st::TermT>(self) -> T {
    pub fn downcast_into<T: std::any::Any>(self) -> T {
        *self.0.downcast::<T>().unwrap()
    }
    pub fn dereferenced<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::RUNTIME_LA.read().unwrap().dereferenced(self.as_ref())?)
    }
    /// If this Value contains dy::Deconstruction, then it calls reconstruct on it, otherwise
    /// returns an error.  The call to reconstruct may return an error.
    // TODO: Figure out if Deconstruction really should be allowed to be made into a Value.
    pub fn reconstruct_in_place(&mut self) -> Result<()> {
        if self.is::<dy::Deconstruction>() {
            // This is a bit silly, but I don't want to bother with unsafe code at this point.
            // The point is to swap out self with a dummy, operate on the dummy (self's former
            // value), and then swap back into place.
            let mut dummy = Value::from(st::Void);
            std::mem::swap(self, &mut dummy);
            // Ideally we would use Box::into_inner here instead of *, but that's somehow still unstable.
            let deconstruction: dy::Deconstruction =
                *dummy.0.downcast::<dy::Deconstruction>().unwrap();
            self.0 = Box::new(deconstruction.reconstruct()?);
        } else {
            // No need to reconstruct anything.
        }
        Ok(())
    }
    fn nondereferencing_abstract_type(&self) -> Self {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .nondereferencing_abstract_type_of(self.as_ref())
            .into()
    }
    fn nondereferencing_is_parametric(&self) -> bool {
        dy::RUNTIME_LA
            .read()
            .unwrap()
            .nondereferencing_is_parametric(self.as_ref())
    }
}

// impl st::DiffT<Value> for Value {
//     type Inverse = Value;
//     // type Error = Error;
//     fn apply_in_place(&self, target: &mut Value) -> Result<()> {
//         dy::RUNTIME_LA
//             .read()
//             .unwrap()
//             .diff_apply_in_place(self.as_ref(), target.as_mut())
//     }
//     fn into_inverse(self) -> Self::Inverse {
//         dy::RUNTIME_LA
//             .read()
//             .unwrap()
//             .diff_into_inverse(self)
//             .expect("TODO: Need to handle this error; probably actually need TryDiff trait.")
//     }
// }
