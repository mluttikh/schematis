//! This module defines various facet types used in XML Schemas.
//!
//! Facets are used to further constrain the value space of a simple type.
//! They provide additional restrictions on the allowed values for elements
//! of that simple type. This module provides definitions for different
//! facet types supported by XML Schemas.
use crate::{Annotation, AnyURI, ID};
use serde::Deserialize;

pub enum Facet<'a> {
    Length(&'a Length),
    MinLength(&'a Length),
    MaxLength(&'a Length),
    Pattern(&'a Pattern),
    WhiteSpace(&'a WhiteSpace),
    Enumeration(&'a Enumeration),
    MinInclusive(&'a BoundaryFacet),
    MaxInclusive(&'a BoundaryFacet),
    MinExclusive(&'a BoundaryFacet),
    MaxExclusive(&'a BoundaryFacet),
    TotalDigits(&'a Digits),
    FractionDigits(&'a Digits),
    Assertion(&'a Assertion),
    ExplicitTimezone(&'a ExplicitTimezone),
}

/// Represents an enumeration facet value used in type restrictions.
///
/// An enumeration facet value (`xs:enumeration`) defines a set of
/// permitted literal values for the content of an element within a
/// type definition. This struct captures the attributes and content
/// associated with an enumeration facet value.
///
/// ```xsd
/// <enumeration
///   id = ID
///   value = anySimpleType
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </enumeration>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enumeration {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:enumeration`). It allows you to specify a
    /// unique identifier for the facet value within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Enumerated value.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction element (`xs:enumeration`). It specifies a literal value
    /// that is allowed for the element content.
    #[serde(rename = "@value")]
    pub value: String,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element (`xs:enumeration`) can
    /// optionally contain an `xs:annotation` child element. This can be
    /// used to provide documentation or other descriptive information
    /// about the permitted values defined by the enumeration.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Enumeration {
    /// Extracts the annotation element from the enumeration facet.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Enumeration` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within enumeration facet definitions
    /// (`xs:enumeration`) to provide documentation or other descriptive
    /// information about the set of allowed literal values for the element
    /// content. This can be helpful in explaining the purpose and meaning
    /// of the permitted values.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents a white space facet value used in type restrictions.
///
/// A white space facet value (`xs:whiteSpace`) defines how white space
/// characters are treated within the content of an element within a
/// type definition. This struct captures the attributes and content
/// associated with a white space facet value.
///
/// ```xsd
/// <whiteSpace
///   fixed = boolean : false
///   id = ID
///   value = (collapse | preserve | replace)
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </whiteSpace>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WhiteSpace {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:whiteSpace`). It allows you to specify a
    /// unique identifier for the facet value within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Fixed value constraint flag (not applicable).
    ///
    /// The `@fixed` attribute is included for consistency with other facet
    /// structs, but it does not have a meaningful effect on white space
    /// handling. It is always implicitly set to `false`.
    #[serde(rename = "@fixed")]
    pub fixed: Option<bool>,
    /// White space handling option.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction element (`xs:whiteSpace`). It specifies how white space
    /// characters are handled. Possible values include:
    ///  * `preserve`: White space characters are preserved as they appear
    ///    in the element content.
    ///  * `collapse`: White space characters are collapsed. Leading and
    ///    trailing white space is removed, and sequences of whitespace
    ///    characters within the content are replaced with a single space
    ///    character.
    ///  * `replace`: All white space characters are replaced with a single
    ///    space character.
    #[serde(rename = "@value")]
    pub value: WhiteSpaceValue,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element (`xs:whiteSpace`) can
    /// optionally contain an `xs:annotation` child element. This can be
    /// used to provide documentation or other descriptive information
    /// about the white space handling option.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum WhiteSpaceValue {
    Preserve,
    Collapse,
    Replace,
}

/// Represents a pattern facet value used in type restrictions.
///
/// A pattern facet value (`xs:pattern`) defines a regular expression
/// that the content of an element within a complex type definition must
/// match. This struct captures the attributes and content associated
/// with a pattern facet value.
///
/// ```xsd
/// <pattern
///   id = ID
///   value = string
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </pattern>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pattern {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:pattern`). It allows you to specify a
    /// unique identifier for the facet value within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// Regular expression pattern.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction element (`xs:pattern`). It specifies the regular
    /// expression that the element content must adhere to. This expression
    /// defines a pattern for valid content based on character sequences
    /// and special characters supported by regular expressions.
    #[serde(rename = "@value")]
    pub value: String,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element (`xs:pattern`) can
    /// optionally contain an `xs:annotation` child element. This can be
    /// used to provide documentation or other descriptive information
    /// about the regular expression pattern.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Pattern {
    /// Extracts the annotation element from the pattern facet.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Pattern` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within pattern facet definitions (`xs:pattern`)
    /// to provide documentation or other descriptive information about the
    /// regular expression pattern applied to an element's content. This can
    /// be particularly helpful in explaining the purpose and structure of the
    /// regular expression, especially for complex patterns.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents a facet value used for decimal digit restrictions.
///
/// A digits facet value (`xs:fractionDigits` or `xs:totalDigits`)
/// defines constraints on the number of decimal digits allowed in a
/// decimal element within a complex type definition. This struct captures
/// the attributes and content associated with a digits facet value.
///
/// This struct can be used to represent either:
///  * Fraction digits - specifies the number of digits allowed to the
///    right of the decimal point (using `xs:fractionDigits`).
///  * Total digits - specifies the total number of digits allowed
///    (including both integer and decimal parts) using `xs:totalDigits`).
///
/// ```xsd
/// <totalDigits
///   fixed = boolean : false
///   id = ID
///   value = positiveInteger
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </totalDigits>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Digits {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction elements (`xs:fractionDigits`, `xs:totalDigits`). It allows
    /// you to specify a unique identifier for the facet value within the
    /// complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Fixed value constraint flag.
    ///
    /// The `@fixed` attribute is an optional attribute on the corresponding
    /// restriction elements. When set to `true`, it indicates that the
    /// specified digits value cannot be changed by further restrictions
    /// derived from this type.
    #[serde(rename = "@fixed")]
    pub fixed: Option<bool>,
    /// Digits facet value.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction elements. It specifies the allowed number of decimal
    /// digits. The value must be a non-negative integer.
    #[serde(rename = "@value")]
    pub value: u32,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element
    /// (`xs:fractionDigits`, `xs:totalDigits`) can optionally contain an
    /// `xs:annotation` child element. This can be used to provide
    /// documentation or other descriptive information about the digits
    /// constraint.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Digits {
    /// Extracts the annotation element from the digits facet.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Digits` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within digits facet definitions (`xs:fractionDigits`,
    /// `xs:totalDigits`) to provide documentation or other descriptive
    /// information about the constraint on the number of decimal digits allowed
    /// in a decimal element. This can help to explain the reasoning behind
    /// the chosen digit limit or any special considerations related to the
    /// constraint.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents a length facet value used in type restrictions.
///
/// This struct is a base type for `MinLength`, `MaxLength`, and `Length`
/// facets, which define constraints on the length of the content of an
/// element within a type definition. The length can be measured in
/// different ways depending on the element's data type:
///  * String-based types (including XML DTD types and anyURI): Length
///    is measured in number of characters.
///  * Binary types: Length is measured in octets of binary data.
///  * List types: Length is measured as the number of items in the list.
///
/// The `value` field of this struct specifies a non-negative integer
/// representing the length restriction. The specific interpretation of this
/// value depends on the context:
///  * `MinLength`: Defines the minimum allowed length.
///  * `MaxLength`: Defines the maximum allowed length.
///  * `Length`: Defines the exact required length.
///
/// It's important to note that `Length` cannot be used in conjunction with
/// either `MinLength` or `MaxLength`. However, both `MinLength` and
/// `MaxLength` can be used together to define a range of allowed lengths.
///
/// ```xsd
/// <length
///   fixed = boolean : false
///   id = ID
///   value = nonNegativeInteger
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </length>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Length {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:length`). It allows you to specify a
    /// unique identifier for the facet value within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// Fixed value constraint flag.
    ///
    /// The `@fixed` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:length`). When set to `true`, it indicates
    /// that the specified length value cannot be changed by further
    /// restrictions derived from this type.
    #[serde(rename = "@fixed")]
    pub fixed: Option<bool>,
    /// Length constraint value.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction element (`xs:minLength`, `xs:maxLength`, `xs:length`).
    /// It specifies the length restriction value. This value must be a
    /// non-negative integer.
    #[serde(rename = "@value")]
    pub value: u32,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element (`xs:length`) can
    /// optionally contain an `xs:annotation` child element. This can be
    /// used to provide documentation or other descriptive information
    /// about the length constraint.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Length {
    /// Extracts the annotation element from the length facet.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Length` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within length facet definitions (`xs:length`)
    /// to provide documentation or other descriptive information about the
    /// character length constraint for a string element. This can help to
    /// explain the reasoning behind the chosen length limit or any special
    /// considerations related to the constraint.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents a boundary facet value used in type restrictions.

/// A boundary facet value (`xs:minInclusive`, `xs:maxInclusive`,
/// `xs:minExclusive`, `xs:maxExclusive`) defines the allowed range for
/// element content within a type definition. This struct captures
/// the attributes and content associated with a boundary facet value.

/// This struct can be used to represent both inclusive and exclusive
/// boundaries depending on the context:
///  * When used with `MinInclusive` or `MaxInclusive`, it defines the
///    minimum or maximum inclusive value for an element (content can be
///    equal to the specified value).
///  * When used with `MinExclusive` or `MaxExclusive`, it defines the
///    minimum or maximum exclusive value for an element (content must be
///    greater than or less than the specified value, respectively).
///
/// ```xsd
/// <boundaryFace
///   fixed = boolean : false
///   id = ID
///   value = anySimpleType
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </boundaryFacet>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BoundaryFacet {
    /// Optional identifier for the facet value.
    ///
    /// The `@id` attribute is an optional attribute on the corresponding
    /// restriction elements (`xs:minInclusive`, `xs:maxInclusive`,
    /// `xs:minExclusive`, `xs:maxExclusive`). It allows you to specify a
    /// unique identifier for the facet value within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// Fixed value constraint flag.
    ///
    /// The `@fixed` attribute is an optional attribute on the corresponding
    /// restriction elements. When set to `true`, it indicates that the
    /// specified facet value cannot be changed by further restrictions
    /// derived from this type.
    #[serde(rename = "@fixed")]
    pub fixed: Option<bool>,
    /// Boundary facet value.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction elements. It specifies the boundary value for the
    /// restriction. The format of the value depends on the data type of
    /// the element it applies to (e.g., numeric literals for numeric types,
    /// date/time strings for date/time types).
    #[serde(rename = "@value")]
    pub value: String,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element
    /// (`xs:minInclusive`, `xs:maxInclusive`, `xs:minExclusive`,
    /// `xs:maxExclusive`) can optionally contain an `xs:annotation` child
    /// element. This can be used to provide documentation or other
    /// descriptive information about the facet value.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl BoundaryFacet {
    /// Extracts the annotation element from the boundary facet.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `BoundaryFacet` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within boundary facet definitions (`xs:minInclusive`,
    /// `xs:maxInclusive`, `xs:minExclusive`, `xs:maxExclusive`) to provide
    /// documentation or other descriptive information about the specific
    /// constraint defined by the facet value. This can help to explain the
    /// reasoning behind the chosen boundary value or any special considerations
    /// related to the constraint.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents an assertion element within an XML Schema.
///
/// An assertion element (`xs:assertion`) is used to define a custom
/// validation constraint within a type definition. The assertion
/// specifies an XPath expression that the instance document must conform
/// to in order to be valid. This struct captures the attributes and
/// content associated with an assertion element.
///
/// ```xsd
/// <assertion
///   id = ID
///   test = an XPath expression
///   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </assertion>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    /// Optional identifier for the assertion element.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:assertion`
    /// element. It allows you to specify a unique identifier for the
    /// assertion within the complex type definition.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// XPath expression defining the validation constraint.
    ///
    /// The `@test` attribute is a required attribute on the `xs:assertion`
    /// element. It specifies an XPath expression that must evaluate to true
    /// for an instance document to be valid. The XPath expression can
    /// reference elements, attributes, and other parts of the instance
    /// document. This expression defines a custom constraint that the
    /// instance data must adhere to.
    #[serde(rename = "@test")]
    pub test: String,
    /// Default namespace for XPath expressions.
    ///
    /// The `@xpathDefaultNamespace` attribute is an optional attribute on
    /// the `xs:assertion` element. It specifies the default namespace URI
    /// to be used when evaluating XPath expressions within the `@test`
    /// attribute. This can help to avoid the need for explicit namespace
    /// prefixes in the XPath expression.
    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<AnyURI>,
    /// Optional annotation element for documentation.
    ///
    /// The body of the `xs:assertion` element can optionally contain an
    /// `xs:annotation` child element. This can be used to provide
    /// documentation or other descriptive information about the assertion.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl Assertion {
    /// Extracts the optional annotation element from the assertion.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Assertion` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within assertions to provide documentation
    /// or other descriptive information about the validation constraint
    /// defined by the assertion's XPath expression. This can help to
    /// clarify the purpose and intent of the assertion.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents an explicit time zone definition in XSD.
///
/// An explicit time zone definition can be used within an XSD schema to
/// specify the time zone offset for date/time data types. This struct
/// captures the attributes and content associated with an explicit time zone
/// definition.
///
/// ```xsd
/// <explicitTimezone
///   fixed = boolean : false
///   id = ID
///   value = NCName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </explicitTimezone>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExplicitTimezone {
    /// Optional identifier for the explicit time zone definition.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:timezone`
    /// element (used for explicit time zones). It allows you to specify
    /// a unique identifier for the time zone definition within the schema.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// Fixed value constraint flag.
    ///
    /// The `@fixed` attribute is an optional attribute on the corresponding
    /// restriction element (`xs:timezone`). When set to `true`, it indicates
    /// that the specified time zone behavior cannot be changed by further
    /// restrictions derived from this type.
    #[serde(rename = "@fixed", default)]
    pub fixed: bool,
    /// Explicit timezone behavior.
    ///
    /// The `@value` attribute is a required attribute on the corresponding
    /// restriction element (`xs:timezone`). It specifies the behavior
    /// regarding explicit time zones for the date/time value.
    #[serde(rename = "@value")]
    pub value: ExplicitTimezoneValue,
    /// Optional annotation element for documentation.
    ///
    /// The body of the corresponding restriction element (`xs:timezone`) can
    /// optionally contain an `xs:annotation` child element. This can be
    /// used to provide documentation or other descriptive information
    /// about the explicit time zone behavior.
    #[serde(rename = "$value")]
    body: Option<Annotation>,
}

impl ExplicitTimezone {
    /// Extracts the annotation element from the explicit time zone definition.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `ExplicitTimezone` struct. If an
    /// annotation element exists, it returns a reference to that element,
    /// otherwise it returns `None`.
    ///
    /// Annotations can be used within explicit time zone definitions to
    /// provide documentation or other descriptive information about the time
    /// zone, such as its source or rationale for use.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ExplicitTimezoneValue {
    Optional,
    Required,
    Prohibited,
}
