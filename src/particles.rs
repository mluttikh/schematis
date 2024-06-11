//! This module defines the various particle types used in XML Schema (XSD) to construct
//! complex type content models.
//!
//! Particles are the fundamental building blocks that specify what elements, groups,
//! wildcards, or other constructs are allowed within an element of a complex type.
//! By combining these particles, you can define rich and expressive content models
//! for your complex types in XSD.
use serde::Deserialize;

use crate::{
    basics::{NCName, QName, ID},
    element_from_body, Annotation, Assert, Block, ComplexType, Final, FormChoice, Key, Keyref,
    ProcessContents, SimpleType, Unique,
};

pub enum Particle<'a> {
    Element(&'a Element),
    Choice(&'a Choice),
    Group(&'a Group),
    Sequence(&'a Sequence),
    Any(&'a Any),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum MaxOccurs {
    Bounded(u32),
    Unbounded(String),
}

/// Represents a sequence particle in an XSD content model.
///
/// A sequence particle specifies an ordered list of elements, groups, or wildcards
/// that must appear in the exact order defined within the complex type content model.
/// Elements within a sequence must appear in the order they are declared.
///
/// You can use sequences to define complex content models for your XSD elements
/// by combining different particle types like element declarations, groups,
/// and wildcards within the sequence.
///
/// ```xsd
/// <sequence
///   id = ID
///   maxOccurs = (nonNegativeInteger | unbounded)  : 1
///   minOccurs = nonNegativeInteger : 1
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (element | group | choice | sequence | any)*)
/// </sequence>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    /// Optional identifier for the sequence particle.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Minimum number of times this sequence must appear (non-negative integer).
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    /// Maximum number of times this sequence can appear.
    // #[serde(default = "some_one_bounded")]
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<MaxOccurs>,
    /// Elements, groups, or wildcards that define the content of the sequence.
    /// The order of elements within this vector is significant and corresponds
    /// to the order in which they must appear in the complex type content model.
    #[serde(rename = "$value", default)]
    body: Vec<SequenceBody>,
}

impl Sequence {
    /// Extracts the optional annotation element from the sequence, if present.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Sequence` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations within a sequence provide comments or metadata for the content model,
    /// but they are not considered part of the actual content structure.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, SequenceBody::Annotation)
    }

    /// Extracts the sequence items as a vector of `Particle` variants.
    ///
    /// This method iterates through the elements within the `body` field
    /// of the `Sequence` and extracts the actual content particles.
    /// It filters out any annotation elements ([Annotation]) and builds a
    /// vector of `Particle` variants based on the encountered particle types
    /// ([Any], [Element], [Group], [Sequence], or [Choice]).
    ///
    /// The resulting vector represents the ordered sequence of elements, groups, or wildcards
    /// that define the content model within the sequence particle.
    pub fn items(&self) -> Vec<Particle> {
        let mut particles = vec![];
        for element in &self.body {
            match element {
                SequenceBody::Any(e) => particles.push(Particle::Any(e)),
                SequenceBody::Annotation(_) => continue,
                SequenceBody::Element(e) => particles.push(Particle::Element(e)),
                SequenceBody::Group(e) => particles.push(Particle::Group(e)),
                SequenceBody::Sequence(e) => particles.push(Particle::Sequence(e)),
                SequenceBody::Choice(e) => particles.push(Particle::Choice(e)),
            }
        }
        particles
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum SequenceBody {
    Any(Any),
    Annotation(Annotation),
    Element(Element),
    Group(Group),
    Sequence(Sequence),
    Choice(Choice),
}

/// Represents an all particle in an XSD content model.
///
/// An all particle specifies a group of elements, groups, or wildcards that must all
/// be present as children of the complex type element. The order of elements within
/// an all particle is not significant, but all the specified elements must be present.
///
/// ```xsd
/// <all
///   id = ID
///   maxOccurs = (0 | 1) : 1
///   minOccurs = (0 | 1) : 1
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (element | any | group)*)
/// </all>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct All {
    /// Optional identifier for the all particle.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Minimum number of times this all particle must appear (non-negative integer).
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    /// Maximum number of times this all particle can appear.
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<u32>,
    /// Elements, groups, or wildcards that define the content of the all particle.
    /// The order within this vector is not significant.
    #[serde(rename = "$value", default)]
    body: Vec<AllBody>,
}

impl All {
    /// Extracts the optional annotation element from the all, if present.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `All` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations within an all provide comments or metadata for the content model,
    /// but they are not considered part of the actual content structure.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, AllBody::Annotation)
    }

    /// Extracts the all items as a vector of `Particle` variants.
    ///
    /// This method iterates through the elements within the `body` field
    /// of the `All` and extracts the actual content particles.
    /// It filters out any annotation elements ([Annotation]) and builds a
    /// vector of `Particle` variants based on the encountered particle types
    /// ([Any], [Element], or [Group]).
    ///
    /// The resulting vector represents the ordered sequence of elements, groups, or wildcards
    /// that define the content model within the all particle.
    pub fn items(&self) -> Vec<Particle> {
        let mut particles = vec![];
        for element in &self.body {
            match element {
                AllBody::Annotation(_) => continue,
                AllBody::Element(e) => particles.push(Particle::Element(e)),
                AllBody::Any(e) => particles.push(Particle::Any(e)),
                AllBody::Group(e) => particles.push(Particle::Group(e)),
            }
        }
        particles
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum AllBody {
    Annotation(Annotation),
    Element(Element),
    Any(Any),
    Group(Group),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Group {
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    #[serde(rename = "@ref")]
    pub r#ref: Option<QName>,
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    // #[serde(default = "some_one_bounded")]
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<MaxOccurs>,
    #[serde(rename = "$value", default)]
    body: Vec<GroupBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum GroupBody {
    All(All),
    Annotation(Annotation),
    Assert(Assert),
    Choice(Choice),
    Sequence(Sequence),
}

/// Represents a choice particle in an XSD content model.
///
/// A choice particle allows one or more elements from a set of alternatives to be present
/// within the complex type element. Only one element from the choices can be present at a time.
/// You can use choice particles to define flexible content models where different elements
/// might be valid depending on the specific context.
///
/// ```xsd
/// <all
///   id = ID
///   maxOccurs = (0 | 1) : 1
///   minOccurs = (0 | 1) : 1
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (element | any | group)*)
/// </all>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Choice {
    /// Optional identifier for the choice particle.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Minimum number of times at least one element from the choices must appear (non-negative integer).
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    /// Maximum number of times an element from the choices can appear.
    // #[serde(default = "some_one_bounded")]
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<MaxOccurs>,
    /// Elements, groups, or other particles that define the available choices within the complex type element.
    #[serde(rename = "$value", default)]
    body: Vec<ChoiceBody>,
}

impl Choice {
    /// Extracts the optional annotation element from the choice, if present.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Choice` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations within a choice provide comments or metadata for the content model,
    /// but they are not considered part of the actual content structure.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ChoiceBody::Annotation)
    }

    /// Extracts the choice items as a vector of `Particle` variants.
    ///
    /// This method iterates through the elements within the `body` field
    /// of the `Choice` and extracts the actual content particles.
    /// It filters out any annotation elements ([Annotation]) and builds a
    /// vector of `Particle` variants based on the encountered particle types
    /// ([Any], [Element], [Group], [Sequence], or [Choice]).
    ///
    /// The resulting vector represents the ordered sequence of elements, groups, or wildcards
    /// that define the content model within the choice particle.
    pub fn items(&self) -> Vec<Particle> {
        let mut particles = vec![];
        for element in &self.body {
            match element {
                ChoiceBody::Any(e) => particles.push(Particle::Any(e)),
                ChoiceBody::Annotation(_) => continue,
                ChoiceBody::Element(e) => particles.push(Particle::Element(e)),
                ChoiceBody::Group(e) => particles.push(Particle::Group(e)),
                ChoiceBody::Sequence(e) => particles.push(Particle::Sequence(e)),
                ChoiceBody::Choice(e) => particles.push(Particle::Choice(e)),
            }
        }
        particles
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ChoiceBody {
    Any(Any),
    Annotation(Annotation),
    Element(Element),
    Group(Group),
    Choice(Choice),
    Sequence(Sequence),
}

/// Represents an "any" particle in an XSD content model.
///
/// An "any" particle allows elements from any namespace to appear within the complex type element,
/// including elements not explicitly declared in the schema. This provides a way to handle unexpected
/// or dynamically generated content. However, it can also loosen the validation constraints
/// of your schema.
///
/// ```xsd
/// <any
///   id = ID
///   maxOccurs = (nonNegativeInteger | unbounded)  : 1
///   minOccurs = nonNegativeInteger : 1
///   namespace = ((##any | ##other) | List of (anyURI | (##targetNamespace | ##local)) )
///   notNamespace = List of (anyURI | (##targetNamespace | ##local))
///   notQName = List of (QName | (##defined | ##definedSibling))
///   processContents = (lax | skip | strict) : strict
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </any>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Any {
    /// Optional identifier for the any particle.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Namespace URI constraint for elements that can be matched.
    ///
    /// The `@namespace` attribute allows you to restrict the allowed namespace for elements that
    /// can match the "any" particle. If set, only elements from the specified namespace can appear.
    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,
    /// Namespace URI constraint for elements that cannot be matched.
    ///
    /// The `@notNamespace` attribute allows you to exclude elements from a specific namespace
    /// from matching the "any" particle. This can be useful in combination with `@namespace`
    /// to restrict allowed elements to a specific namespace while also excluding unwanted elements
    /// from that same namespace.
    #[serde(rename = "@notNamespace")]
    pub not_namespace: Option<String>,
    /// Name constraint for elements that cannot be matched.
    ///
    /// The `@notQName` attribute allows you to exclude elements with a specific qualified name
    /// (combination of namespace prefix and local name) from matching the "any" particle. This
    /// provides more fine-grained control over what elements are allowed or excluded.
    #[serde(rename = "@notQName")]
    pub not_q_name: Option<String>,
    /// Processing mode for wildcard elements.
    ///
    /// The `@processContents` attribute specifies how the content of elements matched by the
    /// "any" particle should be processed. The possible values include `lax` (skip element
    /// validation), `strict` (perform full validation), or `skip` (completely skip the element).
    #[serde(rename = "@processContents")]
    pub process_contents: Option<ProcessContents>,
    /// Minimum number of times this "any" particle must appear (non-negative integer).
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    /// Maximum number of times this "any" particle can appear.
    // #[serde(default = "some_one_bounded")]
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<MaxOccurs>,
    /// Optional annotation element associated with the "any" particle.
    ///
    /// This can be used to provide additional comments or metadata about the wildcard element.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Any {
    /// Extracts the optional annotation element associated with the "any" particle.

    /// This method retrieves the optional `Annotation` element stored within the `body` field
    /// of the `Any` struct. Annotations provide comments or metadata about the wildcard element.

    /// If an annotation is present, this method returns a reference to the contained `Annotation`
    /// struct. Otherwise, it returns `None`.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents an XML Schema element declaration.
///
/// An element declaration in XSD defines the structure and constraints for
/// elements that can appear within an XML document that validates against
/// the schema. This struct captures the various attributes and content
/// associated with an element declaration.
///
/// ```xsd
/// <element
///   abstract = boolean : false
///   block = (#all | List of (extension | restriction | substitution))
///   default = string
///   final = (#all | List of (extension | restriction))
///   fixed = string
///   form = (qualified | unqualified)
///   id = ID
///   maxOccurs = (nonNegativeInteger | unbounded)  : 1
///   minOccurs = nonNegativeInteger : 1
///   name = NCName
///   nillable = boolean : false
///   ref = QName
///   substitutionGroup = List of QName
///   targetNamespace = anyURI
///   type = QName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, ((simpleType | complexType)?, alternative*, (unique | key | keyref)*))
/// </element>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Element {
    /// Optional identifier for the element declaration.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:element`
    /// element. It allows you to specify a unique identifier for the element
    /// declaration within the schema.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Name of the element.
    ///
    /// The `@name` attribute is an optional attribute on the `xs:element`
    /// element. It specifies the name of the element that can appear in
    /// instances of the schema. The name must conform to NCName (Name with
    /// colon) restrictions.
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    /// Nillable flag indicating whether the element can be empty.
    ///
    /// The `@nillable` attribute is an optional attribute on the `xs:element`
    /// element. It specifies whether the element can be empty (have no content).
    /// When set to `true`, the element can appear in an instance with no
    /// child elements or text content.
    #[serde(rename = "@nillable")]
    pub nillable: Option<bool>,
    /// Default value for the element.
    ///
    /// The `@default` attribute is an optional attribute on the `xs:element`
    /// element. It specifies a default value that will be used if no value
    /// is provided for the element in an instance document.
    #[serde(rename = "@default")]
    pub default: Option<String>,
    /// Final declaration restriction.
    ///
    /// The `@final` attribute is an optional attribute on the `xs:element`
    /// element. It specifies whether the element can be derived from by
    /// complex type extensions or restrictions. When set to `true`, the
    /// element cannot be used as a base type for complex type derivations.
    #[serde(rename = "@final")]
    pub r#final: Option<Final>,
    /// Block declaration restricting content model.
    ///
    /// The `@block` attribute is an optional attribute on the `xs:element`
    /// element. It specifies a set of element names that cannot appear as
    /// child elements within the current element. This allows you to restrict
    /// the content model of the element.
    #[serde(rename = "@block")]
    pub block: Option<Vec<Block>>,
    /// Fixed value constraint.
    ///
    /// The `@fixed` attribute is an optional attribute on the `xs:element`
    /// element. It specifies a fixed value that the element must have in
    /// instances of the schema. This enforces a specific value for the element.
    #[serde(rename = "@fixed")]
    pub fixed: Option<String>,
    /// Element form (qualified or unqualified).
    ///
    /// The `@form` attribute is an optional attribute on the `xs:element`
    /// element. It specifies whether the element name must be qualified
    /// (with a namespace prefix) or unqualified (without a prefix) when used
    /// in instances. This is determined by the `elementFormDefault` attribute
    /// on the `schema` element and can be overridden for specific elements.
    #[serde(rename = "@form")]
    pub form: Option<FormChoice>,
    /// Abstract flag for complex types.
    ///
    /// The `@abstract` attribute is an optional attribute on the `xs:element`
    /// element. It is only valid for complex types. When set to `true`, the
    /// element cannot be used directly in instances but can only be used as
    /// a base type for complex type derivations.
    #[serde(rename = "@abstract")]
    pub r#abstract: Option<bool>,
    /// Type reference for element content.
    ///
    /// The `@type` attribute is an optional attribute on the `xs:element`
    /// element. It specifies the type definition that the element content
    /// must conform to. This can be a reference to a named type elsewhere
    /// in the schema or a built-in XML Schema type.
    #[serde(rename = "@type")]
    pub r#type: Option<QName>,
    /// Substitution group for element.
    ///
    /// The `@substitutionGroup` attribute is an optional attribute on the
    /// `xs:element` element. It specifies that the current element belongs
    /// to a substitution group identified by the QName value. This allows
    /// elements from the same substitution group to be used interchangeably
    /// in certain contexts.
    #[serde(rename = "@substitutionGroup")]
    pub substitution_group: Option<QName>,
    /// Minimum occurrence constraint.
    ///
    /// The `@minOccurs` attribute is an optional attribute on the `xs:element`
    /// element. It specifies the minimum number of times the element can
    /// appear in an instance document. The value must be a non-negative
    /// integer.
    #[serde(rename = "@minOccurs")]
    pub min_occurs: Option<u32>,
    /// Maximum occurrence constraint.

    /// The `@maxOccurs` attribute is an optional attribute on the `xs:element`
    /// element. It specifies the maximum number of times the element can
    /// appear in an instance document. The value can be either a non-negative
    /// integer or the special value "unbounded" indicating no upper limit.
    //#[serde(default = "some_one_bounded")]
    #[serde(rename = "@maxOccurs")]
    pub max_occurs: Option<MaxOccurs>,
    /// Reference to another element declaration.
    ///
    /// The `@ref` attribute is an optional attribute on the `xs:element`
    /// element. It specifies a reference to another element declaration
    /// defined elsewhere in the schema. This can be used for element groups
    /// or to reference elements from other schemas through imports or includes.
    #[serde(rename = "@ref")]
    pub r#ref: Option<QName>,
    /// Content elements or groups within the element.
    ///
    /// The body of the `xs:element` element can contain various child
    /// elements that define the content model of the element. This can include
    /// elements like `xs:complexType`, `xs:simpleType`, `xs:annotation`,
    /// and others depending on the specific element type and schema design.
    #[serde(rename = "$value", default)]
    body: Vec<ElementBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ElementBody {
    Annotation(Annotation),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Unique(Unique),
    Key(Key),
    Keyref(Keyref),
    // TODO: Not supported yet
    Alternative,
}
