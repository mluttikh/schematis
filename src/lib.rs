#![allow(dead_code)]
use std::io::BufRead;

use quick_xml::de::Deserializer;
use serde::Deserialize;

pub mod basics;
use basics::{AnyURI, NCName, QName, Token, ID};

pub mod particles;
use particles::{All, Any, Choice, Element, Group, Sequence};

pub mod facets;
use facets::{
    Assertion, BoundaryFacet, Digits, Enumeration, ExplicitTimezone, Facet, Length, Pattern,
    WhiteSpace,
};

#[doc(hidden)]
#[macro_export]
macro_rules! element_from_body {
    ($self:ident, $element_enum:ident::$variant:ident) => {{
        let mut elements = vec![];
        for element in &$self.body {
            if let $element_enum::$variant(e) = element {
                elements.push(e);
            }
        }
        let element = elements.pop();
        if elements.is_empty() {
            element
        } else {
            None
        }
    }};
}

macro_rules! elements_from_body {
    ($self:ident, $element_enum:ident::$variant:ident) => {{
        let mut elements = vec![];
        for element in &$self.body {
            if let $element_enum::$variant(e) = element {
                elements.push(e);
            }
        }
        elements
    }};
}

/// Represents the possible final derivation constraints used in complex types.
///
/// The `final` attribute in XSD complex types allows you to control which
/// derived types can be created from the base type. This enum captures
/// the different options for the `final` attribute value.
///
/// By default, if neither `final` nor `finalDefault` is specified in the
/// schema, there are no restrictions on derivation from that type.
///
/// You can use the `Final` enum to explicitly restrict derived types:
///
///  * `#all`: Prevents derivation by all types (including restriction,
///    extension, list, and union). This overrides any `finalDefault`
///    setting in the schema.
///  * `Extension`: Prevents derivation by extension only. An extension
///    complex type would normally build upon an existing complex type by
///    adding new elements or attributes. Specifying `Extension` in `final`
///    disallows this kind of derivation.
///  * `Restriction`: Prevents derivation by restriction only. A restriction
///    complex type would normally further constrain the content model of an
///    existing complex type by applying facets (e.g., length, pattern) to
///    its elements. Specifying `Restriction` in `final` disallows this type
///    of derivation.
///  * `List`: Prevents derivation by list only. A list complex type
///    represents a sequence of elements of a specific type. Specifying
///    `List` in `final` disallows this kind of derivation.
///  * `Union`: Prevents derivation by union only. A union complex type
///    allows an element to have content that matches the content model of one
///    of several specified types. Specifying `Union` in `final` disallows
///    this type of derivation.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Final {
    #[serde(rename = "#all")]
    All,
    Extension,
    Restriction,
    List,
    Union,
}

/// The document root element of the XML Schema Definition (XSD).
/// It defines the overall structure and characteristics of the XML documents defined by the schema.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct Schema {
    /// Unique identifier for the schema definition.
    ///
    /// The `id` attribute is an optional attribute on the `xs:schema` element
    /// in XSD. It allows you to specify a unique identifier for the schema definition.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    #[serde(rename = "@xmlns")]
    xmlns: Option<String>,
    /// Default attribute form for elements within the schema.
    ///
    /// The `attributeFormDefault` attribute on the `xs:schema` element specifies
    /// the default form (qualified or unqualified) for attributes within the schema.
    #[serde(rename = "@attributeFormDefault")]
    pub attribute_form_default: Option<FormChoice>,
    /// Default element form for elements within the schema.
    ///
    /// The `elementFormDefault` attribute on the `xs:schema` element specifies
    /// the default form (qualified or unqualified) for elements within the schema.
    #[serde(rename = "@elementFormDefault")]
    pub element_form_default: Option<FormChoice>,
    /// Default block restriction for elements within the schema.
    ///
    /// The `blockDefault` attribute on the `xs:schema` element specifies
    /// the default block restriction for elements within the schema.
    #[serde(rename = "@blockDefault")]
    pub block_default: Option<Block>,
    /// Vector of default final restrictions for elements within the schema.
    ///
    /// The `finalDefault` attribute on the `xs:schema` element can specify
    /// a set of default final restrictions that apply to elements within the schema.
    #[serde(rename = "@finalDefault")]
    pub final_default: Option<Vec<Final>>,
    /// Target namespace for the schema definition.
    ///
    /// The `targetNamespace` attribute is a required attribute on the
    /// `xs:schema` element. It specifies the target namespace for the schema
    /// definition. This namespace is used to qualify element and attribute names
    /// within the schema.
    #[serde(rename = "@targetNamespace")]
    pub target_namespace: AnyURI,
    /// Optional version information for the schema.
    ///
    /// The `version` attribute is an optional attribute on the `xs:schema` element.
    /// It allows you to specify a version number or identifier for the schema.
    #[serde(rename = "@version")]
    pub version: Option<Token>,
    /// Optional default attributes for elements within the schema.
    ///
    /// The `defaultAttributes` attribute on the `xs:schema` element is an
    /// optional attribute that can specify a string containing a default set of
    /// attributes to be applied to elements within the schema.
    #[serde(rename = "@defaultAttributes")]
    pub default_attributes: Option<String>,
    /// Optional default namespace for XPath expressions.
    ///
    /// The `xpathDefaultNamespace` attribute on the `xs:schema` element is an
    /// optional attribute that can specify a default namespace to be used for
    /// XPath expressions within the schema.
    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<AnyURI>,
    /// Optional minimum version required for the schema.
    ///
    /// The `minVersion` attribute on the `xs:schema` element is an optional
    /// attribute that can specify a minimum version requirement for software
    /// that processes the schema.
    #[serde(rename = "@minVersion")]
    pub min_version: Option<String>,
    /// Optional language for the schema definition.
    ///
    /// The `xml:lang` attribute is an optional attribute that can be used to
    /// specify the language of the schema definition itself.
    #[serde(rename = "@lang")]
    pub xml_lang: Option<String>,
    #[serde(rename = "$value")]
    body: Vec<SchemaBody>,
}

impl Schema {
    pub fn from_reader(reader: impl BufRead) -> Self {
        let mut deserializer = Deserializer::from_reader(reader);
        Schema::deserialize(&mut deserializer).unwrap()
    }

    /// Extracts all child elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Element]. These elements
    /// represent the actual element definitions within the schema.
    ///
    /// The returned vector contains references to the [Element] structs
    /// defined within the schema. You can use these references to access
    /// information about each element, such as its name, type, and other details.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Element] structs defined
    /// within the schema. If no elements are defined, an empty vector is
    /// returned.
    pub fn elements(&self) -> Vec<&Element> {
        let mut elements = vec![];
        for element in &self.body {
            if let SchemaBody::Element(e) = element {
                elements.push(e.as_ref());
            }
        }

        elements
    }

    /// Extracts all child simple type definitions within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [SimpleType]. These elements
    /// represent the simple type definitions within the schema.
    ///
    /// The returned vector contains references to the [SimpleType] structs
    /// defined within the schema. You can use these references to access
    /// information about each simple type, such as its name, base type, and
    /// any constraints or restrictions applied.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [SimpleType] structs defined
    /// within the schema. If no simple types are defined, an empty vector is
    /// returned.
    pub fn simple_types(&self) -> Vec<&SimpleType> {
        elements_from_body!(self, SchemaBody::SimpleType)
    }

    /// Extracts all child complex type definitions within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [ComplexType]. These elements
    /// represent the complex type definitions within the schema.
    ///
    /// The returned vector contains references to the [ComplexType] structs
    /// defined within the schema. You can use these references to access
    /// information about each complex type, such as its name, structure
    /// (elements and attributes), and any constraints applied.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [ComplexType] structs defined
    /// within the schema. If no complex types are defined, an empty vector is
    /// returned.
    pub fn complex_types(&self) -> Vec<&ComplexType> {
        elements_from_body!(self, SchemaBody::ComplexType)
    }

    /// Extracts all child annotation elements within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Annotation]. These elements
    /// represent annotations associated with various schema components.
    ///
    /// Annotations in XSD can be used to provide additional information,
    /// documentation, or constraints beyond the basic definition of elements,
    /// types, attributes, etc.
    ///
    /// The returned vector contains references to the [Annotation] structs
    /// defined within the schema. You can use these references to access
    /// the content of the annotations, which might be textual or contain
    /// embedded elements.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Annotation] structs defined
    /// within the schema. If no annotations are present, an empty vector is
    /// returned.
    pub fn annotations(&self) -> Vec<&Annotation> {
        elements_from_body!(self, SchemaBody::Annotation)
    }

    /// Extracts all `Include` elements referenced within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Include]. These elements
    /// represent inclusions of other schemas referenced by the current schema.
    ///
    /// The inclusions may reference external XSD documents containing additional
    /// definitions that are incorporated into the current schema.
    ///
    /// The returned vector contains references to the [Include] structs
    /// defined within the schema. You can use these references to access
    /// information about the included schemas, such as their location (URI).
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Include] structs defined
    /// within the schema. If no includes are present, an empty vector is
    /// returned.
    pub fn includes(&self) -> Vec<&Include> {
        elements_from_body!(self, SchemaBody::Include)
    }

    /// Extracts all `Import` elements referenced within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Import]. These elements
    /// represent imports of elements, types, and other definitions from other schemas.
    ///
    /// Imported definitions become available for use within the current schema,
    /// extending its functionality without duplicating definitions.
    ///
    /// The returned vector contains references to the [Import] structs
    /// defined within the schema. You can use these references to access
    /// information about the imported definitions, such as the namespace
    /// and specific elements or types being imported.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Import] structs defined
    /// within the schema. If no imports are present, an empty vector is
    /// returned.
    pub fn imports(&self) -> Vec<&Import> {
        elements_from_body!(self, SchemaBody::Import)
    }

    /// Extracts all `Redefine` elements referenced within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Redefine]. These elements
    /// represent redefinitions of existing elements or types from other schemas
    /// or namespaces.
    ///
    /// Redefinitions allow you to modify or extend the behavior of existing
    /// definitions within the current schema.
    ///
    /// The returned vector contains references to the [Redefine] structs
    /// defined within the schema. You can use these references to access
    /// information about the redefined elements or types, such as their original
    /// source and any modifications applied.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Redefine] structs defined
    /// within the schema. If no redefinitions are present, an empty vector is
    /// returned.
    pub fn redefines(&self) -> Vec<&Redefine> {
        elements_from_body!(self, SchemaBody::Redefine)
    }

    /// Extracts all `Group` elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Group]. These elements
    /// represent groups of elements defined within the schema.
    ///
    /// Groups allow you to define reusable collections of elements that can
    /// be referenced by other elements within the schema. This promotes
    /// code reuse and consistency in element definitions.
    ///
    /// The returned vector contains references to the [Group] structs
    /// defined within the schema. You can use these references to access
    /// information about the elements contained within the group,
    /// their order, and potentially any constraints applied to the group.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Group] structs defined
    /// within the schema. If no groups are defined, an empty vector is
    /// returned.
    pub fn groups(&self) -> Vec<&Group> {
        elements_from_body!(self, SchemaBody::Group)
    }

    /// Extracts all `AttributeGroup` elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [AttributeGroup]. These elements
    /// represent reusable collections of attribute definitions.
    ///
    /// Attribute groups allow you to define a set of attributes that can be
    /// applied to multiple elements within the schema. This promotes code reuse
    /// and consistency in attribute definitions.
    ///
    /// The returned vector contains references to the [AttributeGroup] structs
    /// defined within the schema. You can use these references to access
    /// information about the attributes contained within the group, potentially
    /// including their names, types, and any constraints applied.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [AttributeGroup] structs defined
    /// within the schema. If no attribute groups are defined, an empty vector is
    /// returned.
    pub fn attribute_groups(&self) -> Vec<&AttributeGroup> {
        elements_from_body!(self, SchemaBody::AttributeGroup)
    }

    /// Extracts all `Attribute` elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Attribute]. These elements
    /// represent individual attributes that can be associated with elements
    /// within the schema.
    ///
    /// Attributes provide additional information about elements, allowing you to
    /// define metadata, properties, or constraints that further describe the
    /// element's behavior or usage.
    ///
    /// The returned vector contains references to the [Attribute] structs
    /// defined within the schema. You can use these references to access
    /// information about each attribute, such as its name, data type,
    /// default value (if any), and any constraints applied.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Attribute] structs defined
    /// within the schema. If no attributes are defined, an empty vector is
    /// returned.
    pub fn attributes(&self) -> Vec<&Attribute> {
        elements_from_body!(self, SchemaBody::Attribute)
    }

    /// Extracts all `Notation` elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [Notation]. These elements
    /// represent notations associated with the schema itself or specific components
    /// within the schema.
    ///
    /// Notations in XSD can be used for various purposes, such as:
    ///
    /// * Providing additional documentation or explanations for elements, types,
    ///   attributes, etc.
    /// * Specifying constraints or processing instructions that are not directly
    ///   represented in the schema structure.
    /// * Linking the schema to external resources or documentation.
    ///
    /// The returned vector contains references to the [Notation] structs
    /// defined within the schema. You can use these references to access
    /// information about the notations, including their name, system identifier
    /// (if present), and the content of the notation (which might be textual
    /// or contain embedded elements).
    ///
    /// # Returns
    ///
    /// A vector containing references to all [Notation] structs defined
    /// within the schema. If no notations are present, an empty vector is
    /// returned.
    pub fn notations(&self) -> Vec<&Notation> {
        elements_from_body!(self, SchemaBody::Notation)
    }

    /// Extracts all `DefaultOpenContent` elements defined within the schema.
    ///
    /// This method iterates through the schema's body elements (if present)
    /// and collects all elements of type [DefaultOpenContent].
    /// These elements represent the default openness constraint for elements
    /// within the schema.
    ///
    /// The openness constraint specifies whether an element can contain elements
    /// from other complex types within the schema or not.
    ///
    /// * `Open`: The element can contain elements from any other complex type.
    /// * `Closed`: The element can only contain elements explicitly defined
    ///   within its own content model.
    ///
    /// If no `DefaultOpenContent` element is present within the schema,
    /// a closed content model is assumed by default.
    ///
    /// The returned vector contains references to the [DefaultOpenContent] structs
    /// defined within the schema. You can use these references to access
    /// information about the openness constraint, such as whether it's open
    /// or closed.
    ///
    /// # Returns
    ///
    /// A vector containing references to all [DefaultOpenContent] structs defined
    /// within the schema. If no [DefaultOpenContent] elements are present,
    /// an empty vector is returned.
    pub fn default_open_contents(&self) -> Vec<&DefaultOpenContent> {
        elements_from_body!(self, SchemaBody::DefaultOpenContent)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum FormChoice {
    Qualified,
    Unqualified,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Block {
    #[serde(rename = "#all")]
    All,
    Extension,
    Restriction,
    Substitution,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum SchemaBody {
    Include(Include),
    Import(Import),
    Override,
    Redefine(Redefine),
    Annotation(Annotation),
    DefaultOpenContent(DefaultOpenContent),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Group(Group),
    AttributeGroup(AttributeGroup),
    Element(Box<Element>),
    Attribute(Attribute),
    Notation(Notation),
}

/// Represents an XML Schema include element.
///
/// An include element in XSD allows you to include the contents of another
/// schema document that has the same target namespace. This is useful for modularizing
/// complex schema definitions and reusing common components across multiple
/// schemas.
///
/// ```xs
/// <include
///   id = ID
///   schemaLocation = anyURI
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </include>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Include {
    /// Optional identifier for the include element.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:include` element.
    /// It allows you to specify a unique identifier for the include element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Location of the included schema document.
    ///
    /// The `@schemaLocation` attribute is a required attribute on the
    /// `xs:include` element. It specifies the URI (Uniform Resource Identifier)
    /// of the schema document to be included.
    #[serde(rename = "@schemaLocation")]
    pub schema_location: AnyURI,
    /// Optional annotations associated with the include element.
    ///
    /// The body of the `xs:include` element can optionally contain annotation
    /// elements that provide comments or documentation for the inclusion.
    #[serde(rename = "$value", default)]
    pub annotations: Vec<Annotation>,
}

/// Represents an XML Schema import element.
///
/// An import element in XSD allows you to import definitions from another
/// schema document into the current namespace. This is similar to include,
/// but it creates a namespace alias for the imported definitions. This allows
/// you to reference elements and types from the imported schema using the
/// specified namespace prefix.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Import {
    /// Optional identifier for the import element.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:import` element.
    /// It allows you to specify a unique identifier for the import element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Namespace of the imported schema.
    ///
    /// The `@namespace` attribute is an optional attribute on the `xs:import`
    /// element. It specifies the namespace of the schema being imported.
    /// If not specified, the target namespace of the imported schema is used.
    #[serde(rename = "@namespace")]
    pub namespace: Option<AnyURI>,
    /// Location of the imported schema document.
    ///
    /// The `@schemaLocation` attribute is a required attribute on the
    /// `xs:import` element. It specifies the URI (Uniform Resource Identifier)
    /// of the schema document to be imported.
    #[serde(rename = "@schemaLocation")]
    pub schema_location: AnyURI,
    /// Optional annotations associated with the import element.
    ///
    /// The body of the `xs:import` element can optionally contain annotation
    /// elements that provide comments or documentation for the import.
    #[serde(rename = "$value", default)]
    pub annotations: Vec<Annotation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Redefine {
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    #[serde(rename = "@schemaLocation")]
    pub schema_location: AnyURI,
    #[serde(rename = "$value", default)]
    body: Vec<RedefineBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum RedefineBody {
    Annotation(Annotation),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Group(Group),
    AttributeGroup(AttributeGroup),
}

/// Represents an XSD notation declaration within the schema. This struct
/// corresponds to the `<xsd:notation>` element in the XSD. Notations
/// provide a way to define external systems for processing data within an
/// XML document.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Notation {
    #[serde(rename = "@id")]
    id: Option<ID>,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@public")]
    public: String,
    #[serde(rename = "@system")]
    system: Option<String>,
}

/// Represents an XSD attribute group definition within the schema. This struct
/// corresponds to the `<xsd:attributeGroup>` element in the XSD. Attribute
/// groups allow grouping frequently used attribute definitions for reuse
/// across elements within the schema.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AttributeGroup {
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    #[serde(rename = "@ref")]
    pub r#ref: Option<QName>,
    #[serde(rename = "$value", default)]
    body: Vec<AttributeGroupBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum AttributeGroupBody {
    Annotation(Annotation),
    Attribute(Attribute),
    AnyAttribute(AnyAttribute),
    AttributeGroup(AttributeGroup),
}

/// Represents an XML Schema attribute declaration.
///
/// An attribute declaration in XSD defines the structure and constraints
/// for attributes that can be associated with elements within an XML document
/// that validates against the schema. This struct captures the various
/// attributes and content associated with an attribute declaration.
///
/// ```xsd
/// <attribute
///   default = string
///   fixed = string
///   form = (qualified | unqualified)
///   id = ID
///   name = NCName
///   ref = QName
///   targetNamespace = anyURI
///   type = QName
///   use = (optional | prohibited | required) : optional
///   inheritable = boolean
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, simpleType?)
/// </attribute>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    /// Optional identifier for the attribute declaration.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:attribute`
    /// element. It allows you to specify a unique identifier for the attribute
    /// declaration within the schema.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Name of the attribute.
    ///
    /// The `@name` attribute is a required attribute on the `xs:attribute`
    /// element. It specifies the name of the attribute that can be associated
    /// with elements in instances of the schema. The name must conform to
    /// NCName (Name with colon) restrictions.
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    /// Type reference for attribute content.
    ///
    /// The `@type` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies the type definition that the attribute content
    /// must conform to. This can be a reference to a named type elsewhere
    /// in the schema or a built-in XML Schema type.
    #[serde(rename = "@type")]
    pub r#type: Option<QName>,
    /// Use constraint (optional, required, prohibited).
    ///
    /// The `@use` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies whether the attribute is optional, required,
    /// or prohibited for elements that can have this attribute.
    #[serde(rename = "@use")]
    pub r#use: Option<AttributeUse>,
    /// Reference to another attribute declaration.
    ///
    /// The `@ref` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies a reference to another attribute declaration
    /// defined elsewhere in the schema. This can be used for attribute groups
    /// or to reference attributes from other schemas through imports or includes.
    #[serde(rename = "@ref")]
    pub r#ref: Option<QName>,
    /// Default value for the attribute.
    ///
    /// The `@default` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies a default value that will be used if no value
    /// is provided for the attribute in an instance document.
    #[serde(rename = "@default")]
    pub default: Option<String>,
    /// Fixed value constraint.
    ///
    /// The `@fixed` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies a fixed value that the attribute must have in
    /// instances of the schema. This enforces a specific value for the attribute.
    #[serde(rename = "@fixed")]
    pub fixed: Option<String>,
    /// Attribute form (qualified or unqualified).
    ///
    /// The `@form` attribute is an optional attribute on the `xs:attribute`
    /// element. It specifies whether the attribute name must be qualified
    /// (with a namespace prefix) or unqualified (without a prefix) when used
    /// in instances. This is determined by the `elementFormDefault` attribute
    /// on the `schema` element and can be overridden for specific attributes.
    #[serde(rename = "@form")]
    pub form: Option<FormChoice>,
    /// Namespace the attribute belongs to.
    ///
    /// The `@targetNamespace` attribute is an optional attribute on the
    /// `xs:attribute` element. It specifies the namespace URI that the
    /// attribute belongs to. This is important for qualified attribute names
    /// and resolving namespace prefixes.
    #[serde(rename = "@targetNamespace")]
    pub target_namespace: Option<AnyURI>,
    /// Inheritance flag for attribute groups.
    ///
    /// The `@inheritable` attribute is an optional attribute on the
    /// `xs:attribute` element. It is only relevant when used within an
    /// attribute group definition. When set to `true`, the attribute is
    /// inherited by elements that reference the attribute group.
    #[serde(rename = "@inheritable")]
    pub inheritable: Option<bool>,
    /// Content elements or groups within the attribute.
    ///
    /// The body of the `xs:attribute` element can optionally contain
    /// child elements that define additional aspects of the attribute,
    /// such as an annotation element for documentation.
    #[serde(rename = "$value", default)]
    body: Vec<AttributeBody>,
}

impl Attribute {
    /// Extracts the annotation element from the attribute body.
    ///
    /// This method iterates through the body of the attribute and
    /// collects all elements of type [Annotation]. If found, it returns
    /// a reference to that annotation element, otherwise it returns `None`.
    ///
    /// Annotations can be used within attribute declarations to provide
    /// documentation or other descriptive information about the attribute.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, AttributeBody::Annotation)
    }

    /// Extracts the simple type element from the attribute body.
    ///
    /// This method iterates through the body of the attribute and
    /// collects all element of type [SimpleType]. If found, it returns
    /// a reference to that simple type element, otherwise it returns `None`.
    ///
    /// Simple type definitions can be used within attribute declarations to
    /// specify the allowed values and constraints for the attribute content.
    pub fn simple_type(&self) -> Option<&SimpleType> {
        element_from_body!(self, AttributeBody::SimpleType)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeUse {
    Optional,
    Prohibited,
    Required,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum AttributeBody {
    Annotation(Annotation),
    SimpleType(SimpleType),
}

/// Represents the `defaultOpenContent` element within an XSD complex type definition.
///
/// The `defaultOpenContent` element specifies the default allowed content for elements of the
/// complex type when no explicit content model is defined through particle declarations. This
/// allows you to control what elements are allowed as children of the complex type element
/// by default.
///
/// ```xsd
/// <defaultOpenContent
///   appliesToEmpty = boolean : false
///   id = ID
///   mode = (interleave | suffix) : interleave
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, any)
/// </defaultOpenContent>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DefaultOpenContent {
    /// Optional identifier for the `defaultOpenContent` element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Open content mode for the complex type.
    ///
    /// The `@mode` attribute specifies how elements from outside the schema can be mixed
    /// with elements declared within the schema for this complex type. This is relevant when
    /// using wildcard elements (`Any`) within the `defaultOpenContent` definition.
    ///
    /// The possible values for `@mode` are:
    ///
    /// * `Interleave`: Allows elements from any namespace to appear interleaved with elements
    ///   declared within the schema for this complex type. This means elements from any
    ///   namespace can be mixed freely with elements declared in the schema as children
    ///   of the complex type element.
    /// * `Suffix`: Allows elements from any namespace to appear only after elements declared
    ///   within the schema for this complex type. This enforces that elements declared in the
    ///   schema must appear first, followed by any additional elements from other namespaces.
    ///
    /// The choice of mode depends on the desired structure and validation for the complex type content.
    /// `Interleave` provides more flexibility for mixing elements, while `Suffix` ensures a
    /// specific order and stricter validation for elements declared in the schema.
    #[serde(rename = "@mode")]
    pub mode: Option<OpenContentMode>,
    /// Applicability of open content to empty elements.
    ///
    /// The `@appliesToEmpty` attribute controls whether the open content applies to empty elements
    /// of the complex type. If set to `true`, the open content allows any elements even if the
    /// complex type element has no child elements explicitly declared.
    #[serde(rename = "@appliesToEmpty")]
    pub applies_to_empty: Option<bool>,
    #[serde(rename = "$value")]
    body: Vec<OpenContentBody>,
}

impl DefaultOpenContent {
    /// Extracts the optional annotation element from the `defaultOpenContent` body.
    ///
    /// This method searches for an `Annotation` element within the `body` vector of the
    /// `DefaultOpenContent` struct. If an `Annotation` element is present, this method
    /// returns a reference to the contained `Annotation` struct. Otherwise, it returns `None`.
    ///
    /// This method is useful for retrieving any comments or metadata associated with the
    /// `defaultOpenContent` definition.
    pub fn annotations(&self) -> Option<&Annotation> {
        element_from_body!(self, OpenContentBody::Annotation)
    }

    /// Extracts the optional "any" element from the `defaultOpenContent` body.
    ///
    /// This method searches for an `Any` element within the `body` vector of the
    /// `DefaultOpenContent` struct. If an `Any` element is present, this method returns a
    /// reference to the contained `Any` struct. Otherwise, it returns `None`.
    ///
    /// An "any" element allows elements from any namespace to appear within the complex type
    /// content, potentially loosening validation constraints. This method is useful for
    /// accessing the wildcard element definition if present within the `defaultOpenContent`.
    pub fn any(&self) -> Option<&Any> {
        element_from_body!(self, OpenContentBody::Any)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum OpenContentBody {
    Any(Any),
    Annotation(Annotation),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum OpenContentMode {
    Interleave,
    Suffix,
}

/// Represents an XML Schema simple type definition.
///
/// A simple type definition in XSD specifies a set of constraints
/// that the content of an element or attribute must conform to.
/// This struct captures the various attributes and content associated
/// with a simple type definition.
///
/// ```xsd
/// <simpleType
///   final = (#all | List of (list | union | restriction | extension))
///   id = ID
///   name = NCName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (restriction | list | union))
/// </simpleType>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleType {
    /// Optional identifier for the simple type definition.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:simpleType`
    /// element. It allows you to specify a unique identifier for the simple
    /// type definition within the schema.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Final declaration restriction for the simple type.
    ///
    /// The `@final` attribute is an optional attribute on the `xs:simpleType`
    /// element. It specifies whether the simple type can be derived from by
    /// restriction. When set to `true`, the simple type cannot be used as a
    /// base type for further type restrictions.
    #[serde(rename = "@final")]
    pub r#final: Option<Final>,
    /// Name of the simple type definition.
    ///
    /// The `@name` attribute is an optional attribute on the `xs:simpleType`
    /// element. It specifies a name for the simple type definition. This name
    /// can be used to refer to the simple type elsewhere in the schema.
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    /// Content elements or groups within the simple type definition.
    ///
    /// The body of the `xs:simpleType` element can contain various child
    /// elements that define the specific constraints and content model for
    /// the simple type. These elements can include things like restrictions,
    /// lists, unions, and built-in types.
    #[serde(rename = "$value", default)]
    body: Vec<SimpleTypeBody>,
}

impl SimpleType {
    /// Retrieves the optional annotation associated with the `SimpleType`.
    ///
    /// Simple types in XSD can have an optional annotation element that
    /// provides additional metadata or documentation for the type itself.
    /// This annotation can contain human-readable text or other elements
    /// for richer descriptions.
    ///
    /// This method iterates through the `body` elements of the `SimpleType`
    /// (if present) and searches for elements of type [Annotation].
    /// If exactly one annotation element is found, it is returned as an `Option<&Annotation>`.
    ///
    /// If no annotation element is present or there are multiple annotations
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Some(&Annotation)` if a single annotation element is found.
    /// * `None` if no annotation is present or there are multiple annotations.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, SimpleTypeBody::Annotation)
    }

    /// Retrieves the content of the `SimpleType`.
    ///
    /// Simple types in XSD must have one of three content elements: restriction,
    /// union, or list. These elements define the allowed values and structure
    /// of the simple type.
    ///
    /// * Restriction: Defines a subset of an existing simple type by imposing
    ///   facets that restrict the allowed values.
    /// * Union: Combines multiple simple types into a single type, allowing
    ///   values from any of the constituent types.
    /// * List: Creates a list type from another simple type, allowing for
    ///   sequences of values from the base type.
    ///
    /// Returns:
    ///  * `Ok(SimpleTypeContent)` containing the content type (Restriction, Union, or List)
    ///    if found.
    ///  * `Err(String)` containing an error message if the SimpleType has
    ///    no valid content or only contains an Annotation element (this violates the XSD specification).
    pub fn content(&self) -> Result<SimpleTypeContent, String> {
        for element in &self.body {
            match element {
                SimpleTypeBody::Annotation(_) => continue,
                SimpleTypeBody::Restriction(e) => return Ok(SimpleTypeContent::Restriction(e)),
                SimpleTypeBody::Union(e) => return Ok(SimpleTypeContent::Union(e)),
                SimpleTypeBody::List(e) => return Ok(SimpleTypeContent::List(e)),
            };
        }
        // TODO: Replace this error with a proper error type
        Err("SimpleType has no valid content (restriction, union, or list)".to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum SimpleTypeBody {
    Annotation(Annotation),
    Restriction(Restriction),
    Union(Union),
    List(List),
}

pub enum SimpleTypeContent<'a> {
    Restriction(&'a Restriction),
    Union(&'a Union),
    List(&'a List),
}

/// Represents a union type in an XML Schema.
///
/// A union type allows an element to have content that matches the
/// content model of one of several specified types. This struct captures
/// the definition of a union type.
///
/// ```xsd
/// <union
///   id = ID
///   memberTypes = List of QName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, simpleType*)
/// </union>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Union {
    /// Optional identifier for the union type.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:union`
    /// element. It allows you to specify a unique identifier for the union
    /// complex type within the schema.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// List of allowed member types for the union (if specified).
    ///
    /// The `@memberTypes` attribute is an optional attribute on the
    /// `xs:union` element. It specifies a list of qualified names (QNames)
    /// representing the allowed member types for the union. If present, an
    /// element with a union type can only have content that matches the content
    /// model of one of the types listed in `member_types`.
    #[serde(rename = "@memberTypes")]
    pub member_types: Option<Vec<QName>>,
    #[serde(rename = "$value", default)]
    body: Vec<UnionBody>,
}

impl Union {
    /// Extracts the annotation element from the union body.

    /// This method iterates through the body of the union and
    /// collects all elements of type [Annotation]. If found, it returns
    /// a reference to that annotation element, otherwise, it returns `None`.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, UnionBody::Annotation)
    }

    /// Extracts all simple type elements from the union body.
    ///
    /// This method iterates through the body of the union and
    /// collects all elements of type [SimpleType]. If found, it returns
    /// a reference to that simple type element, otherwise it returns `None`.
    ///
    /// The member types of a union can be defined anonymously within the union
    /// as simple type children.
    pub fn simple_types(&self) -> Vec<&SimpleType> {
        elements_from_body!(self, UnionBody::SimpleType)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum UnionBody {
    SimpleType(SimpleType),
    Annotation(Annotation),
}

/// Represents a list complex type in an XML Schema.
///
/// A list type defines a sequence of elements of a specific type.
/// This struct captures the definition of a list complex type.
///
/// ```xsd
/// <list
///   id = ID
///   itemType = QName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, simpleType?)
/// </list>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct List {
    /// Optional identifier for the list complex type.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:list`
    /// element. It allows you to specify a unique identifier for the list
    /// complex type within the schema.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Optional name of the item type for the list.
    ///
    /// The `@itemType` attribute is an optional attribute on the `xs:list`
    /// element. It specifies the qualified name (QName) of the simple type
    /// that the elements within the list must conform to.
    #[serde(rename = "@itemType")]
    pub item_type: Option<QName>,
    /// Optional annotation elements for documentation.
    ///
    /// The body of the `xs:list` element can optionally contain one or
    /// more child elements that provide more information about the list.
    /// This field stores these elements.
    ///
    /// The possible elements within the body can be:
    ///
    /// * Annotations (`xs:annotation` element): These elements can be used
    ///   to provide documentation or other descriptive information about the
    ///   list complex type.
    /// * Simple type references (`SimpleType` structs): These can occur if
    ///   the item type is not explicitly specified in the `@itemType`
    ///   attribute. In such cases, the body can contain references to the
    ///   allowed simple type.
    #[serde(rename = "$value", default)]
    body: Vec<ListBody>,
}

impl List {
    /// Extracts the annotation element from the list body.

    /// This method iterates through the body of the list and
    /// collects all elements of type [Annotation]. If found, it returns
    /// a reference to that annotation element, otherwise, it returns `None`.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ListBody::Annotation)
    }

    /// Extracts all simple type elements from the list body.
    ///
    /// This method iterates through the body of the list and
    /// collects all elements of type [SimpleType]. If found, it returns
    /// a reference to that simple type element, otherwise it returns `None`.
    ///
    /// The member types of a list can be defined anonymously within the list
    /// as simple type children.
    pub fn simple_types(&self) -> Vec<&SimpleType> {
        elements_from_body!(self, ListBody::SimpleType)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ListBody {
    SimpleType(SimpleType),
    Annotation(Annotation),
}

/// Represents an XML Schema restriction element.
///
/// A restriction element in XSD allows you to define a subset of an existing
/// simple type by imposing facets that restrict the allowed values. These facets
/// can further constrain the data type, format, length, or other characteristics
/// of the values allowed within the simple type.
///
/// ```xsd
/// <restriction
///   base = QName
///   id = ID
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (simpleType?, (minExclusive | minInclusive | maxExclusive | maxInclusive | totalDigits | fractionDigits | length | minLength | maxLength | enumeration | whiteSpace | pattern | assertion | {any with namespace: ##other})*)?, ((attribute | attributeGroup)*, anyAttribute?), assert*)
/// </restriction>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Restriction {
    /// Optional identifier for the restriction element.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:restriction`
    /// element. It allows you to specify a unique identifier for the restriction.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Base type for the restriction.
    ///
    /// The `@base` attribute is a required attribute on the `xs:restriction`
    /// element. It specifies the simple type that this restriction is based on.
    #[serde(rename = "@base")]
    pub base: Option<QName>,
    /// Facets or elements defining the restriction details.
    ///
    /// The body of the `xs:restriction` element can contain various facets or
    /// elements that define the specific restrictions applied to the base type.
    /// These can include elements like `minLength`, `maxLength`, `pattern`,
    /// and others depending on the base type.
    #[serde(rename = "$value", default)]
    body: Vec<RestrictionBody>,
}

impl Restriction {
    /// Extracts the annotation element from the restriction.
    ///
    /// This method retrieves the optional `xs:annotation` child element
    /// from the `body` field of the `Restriction` struct. If an annotation
    /// element exists, it returns a reference to that element, otherwise
    /// it returns `None`.
    ///
    /// Annotations can be used within restriction elements to provide
    /// documentation or other descriptive information about the overall
    /// constraints applied to the element content. This can be helpful in
    /// understanding the purpose and rationale behind the defined restrictions.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, RestrictionBody::Annotation)
    }

    /// Extracts the possible `xs:simpleType` element from the restriction.
    ///
    /// This method checks if the `body` field of the `Restriction` struct
    /// contains a `SimpleType` element. If it does, it returns a reference
    /// to that element, otherwise it returns `None`.
    ///
    /// A `simpleType` element within a restriction can define a new simple
    /// type based on an existing base type with additional constraints.
    /// This allows for creating specialized simple types tailored to specific
    /// content requirements.
    pub fn simple_type(&self) -> Option<&SimpleType> {
        element_from_body!(self, RestrictionBody::SimpleType)
    }

    /// Extracts all `xs:assert` elements from the restriction.
    ///
    /// This method iterates over the `body` field of the `Restriction` struct
    /// and collects all elements of type [Assert]. It returns a vector containing
    /// references to all found `Assert` elements, or an empty vector if none
    /// are present.
    ///
    /// Assert elements within a restriction can define specific conditions
    /// that the element content must satisfy. These can be used to enforce
    /// additional validation rules beyond the constraints defined by the base
    /// type or other facets within the restriction.
    pub fn asserts(&self) -> Vec<&Assert> {
        elements_from_body!(self, RestrictionBody::Assert)
    }

    /// Extracts the list of facets associated with the restriction.
    ///
    /// This method iterates through the elements within the `body` field
    /// of the `Restriction` struct and extracts the defined facets. It
    /// filters out elements that are not relevant to facets
    /// (e.g., annotations) and builds a vector of `Facet` variants
    /// corresponding to the identified restriction types.
    ///
    /// The returned vector contains the different facets applied to the
    /// element content. Each facet enforces a specific constraint, such as
    /// minimum/maximum length, allowed patterns, or enumeration of valid
    /// values.
    pub fn facets(&self) -> Vec<Facet> {
        let mut elements = vec![];
        for element in &self.body {
            match element {
                RestrictionBody::Pattern(e) => elements.push(Facet::Pattern(e)),
                RestrictionBody::Length(e) => elements.push(Facet::Length(e)),
                RestrictionBody::Annotation(_) => continue,
                RestrictionBody::WhiteSpace(e) => elements.push(Facet::WhiteSpace(e)),
                RestrictionBody::SimpleType(_) => continue,
                RestrictionBody::AnyAttribute(_) => continue,
                RestrictionBody::MinInclusive(e) => elements.push(Facet::MinExclusive(e)),
                RestrictionBody::MaxInclusive(e) => elements.push(Facet::MaxInclusive(e)),
                RestrictionBody::MinExclusive(e) => elements.push(Facet::MinExclusive(e)),
                RestrictionBody::MaxExclusive(e) => elements.push(Facet::MaxExclusive(e)),
                RestrictionBody::MinLength(e) => elements.push(Facet::MinLength(e)),
                RestrictionBody::MaxLength(e) => elements.push(Facet::MaxLength(e)),
                RestrictionBody::FractionDigits(e) => elements.push(Facet::FractionDigits(e)),
                RestrictionBody::TotalDigits(e) => elements.push(Facet::TotalDigits(e)),
                RestrictionBody::Enumeration(e) => elements.push(Facet::Enumeration(e)),
                RestrictionBody::Sequence(_) => continue,
                RestrictionBody::Attribute(_) => continue,
                RestrictionBody::AttributeGroup(_) => continue,
                RestrictionBody::Group(_) => continue,
                RestrictionBody::All(_) => continue,
                RestrictionBody::Choice(_) => continue,
                RestrictionBody::Assertion(e) => elements.push(Facet::Assertion(e)),
                RestrictionBody::ExplicitTimezone(e) => elements.push(Facet::ExplicitTimezone(e)),
                RestrictionBody::Assert(_) => continue,
            }
        }
        elements
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum RestrictionBody {
    Pattern(Pattern),
    Length(Length),
    Annotation(Annotation),
    WhiteSpace(WhiteSpace),
    SimpleType(SimpleType),
    AnyAttribute(AnyAttribute),
    MinInclusive(BoundaryFacet),
    MaxInclusive(BoundaryFacet),
    MinExclusive(BoundaryFacet),
    MaxExclusive(BoundaryFacet),
    MinLength(Length),
    MaxLength(Length),
    FractionDigits(Digits),
    TotalDigits(Digits),
    Enumeration(Enumeration),
    Sequence(Sequence),
    Attribute(Attribute),
    AttributeGroup(AttributeGroup),
    Group(Group),
    All(All),
    Choice(Choice),
    Assertion(Assertion),
    ExplicitTimezone(ExplicitTimezone),
    Assert(Assert),
}

/// Represents an `anyAttribute` element within an XSD complex type definition.
///
/// The `anyAttribute` element allows attributes from any namespace to be present on elements
/// of the complex type. This provides flexibility in defining the allowed attributes for the
/// complex type but can also loosen validation constraints.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnyAttribute {
    /// Optional identifier for the `anyAttribute` element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Namespace URI constraint for allowed attributes.
    ///
    /// The `@namespace` attribute allows you to restrict the allowed namespace for attributes
    /// that can appear on the element. If set, only attributes from the specified namespace
    /// can be present.
    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,
    /// Namespace URI constraint for excluded attributes.
    ///
    /// The `@notNamespace` attribute allows you to exclude attributes from a specific namespace
    /// from being present on the element. This can be useful in combination with `@namespace`
    /// to restrict allowed attributes to a specific namespace while also excluding unwanted
    /// attributes from that same namespace.
    #[serde(rename = "@notNamespace")]
    pub not_namespace: Option<String>,
    /// Name constraint for excluded attributes.
    ///
    /// The `@notQName` attribute allows you to exclude attributes with a specific qualified name
    /// (combination of namespace prefix and local name) from being present on the element. This
    /// provides more fine-grained control over what attributes are allowed or excluded.
    #[serde(rename = "@notQName")]
    pub not_q_name: Option<String>,
    /// Processing mode for wildcard attributes.
    ///
    /// The `@processContents` attribute specifies how the content of attribute values that match
    /// the `anyAttribute` wildcard should be processed. The possible values include `lax` (skip
    /// attribute value validation), `strict` (perform full validation), or `skip` (completely skip
    /// the attribute value).
    #[serde(rename = "@processContents")]
    pub process_contents: Option<ProcessContents>,
    /// Optional annotation element associated with the `anyAttribute`.
    ///
    /// This can be used to provide additional comments or metadata about the wildcard attribute
    /// definition.
    #[serde(rename = "$value", default)]
    body: Option<Annotation>,
}

impl AnyAttribute {
    /// Extracts the optional annotation element associated with the "anyAttribute".

    /// This method retrieves the optional `Annotation` element stored within the `body` field
    /// of the `AnyAttribute` struct. Annotations provide comments or metadata about the wildcard element.

    /// If an annotation is present, this method returns a reference to the contained `Annotation`
    /// struct. Otherwise, it returns `None`.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ProcessContents {
    Lax,
    Strict,
    Skip,
}

/// Represents a complex type definition within an XSD schema.
///
/// Complex types are used to define reusable element structures with specific content models.
/// They can contain elements, attributes, attribute groups, and other components to define
/// the allowed content and structure of an element.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexType {
    // Optional identifier for the complex type.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Name of the complex type.
    #[serde(rename = "@name")]
    pub name: Option<NCName>,
    /// Mixed content model flag.
    ///
    /// The `@mixed` attribute specifies whether the complex type allows elements and character
    /// data (text) to be mixed within its content. If set to `true`, both elements and text
    /// can appear as children of the element using this complex type.
    #[serde(rename = "@mixed")]
    pub mixed: Option<bool>,
    /// Derivation restrictions (final derivation set).
    ///
    /// The `@final` attribute specifies a set of types from which the current complex type cannot
    /// be further derived. This helps control inheritance relationships within the schema.
    #[serde(rename = "@final")]
    pub r#final: Option<Vec<Final>>,
    /// Block inheritance restrictions.
    ///
    /// The `@block` attribute specifies a set of types that cannot be derived from the current
    /// complex type. This helps control inheritance relationships and prevent specific types
    /// from being used as base types.
    #[serde(rename = "@block")]
    pub block: Option<Vec<Block>>,
    /// Abstract complex type flag.
    ///
    /// The `@abstract` attribute indicates whether the complex type is abstract. Abstract types
    /// cannot be used as element types themselves but can be used as base types for other complex
    /// types.
    #[serde(rename = "@abstract")]
    pub r#abstract: Option<bool>,
    /// Base type of the complex type (if derived).
    ///
    /// The `@type` attribute specifies the base type from which the current complex type derives.
    /// This allows for inheritance and building complex types on top of existing ones.
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    /// Whether default attribute applies from base type.
    ///
    /// The `@default_attributes_apply` attribute controls whether default attribute values from
    /// the base type are inherited by elements using this complex type. If set to `false`,
    /// default attribute values are not inherited.
    #[serde(rename = "@default_attributes_apply")]
    pub default_attributes_apply: Option<bool>,
    /// Content model definition for the complex type.
    ///
    /// The `body` vector contains elements that define the content model of the complex type.
    /// This can include various components like `All`, `Sequence`, `Choice`, `Attribute`,
    /// and others, specifying the allowed elements, attributes, and their order within the
    /// complex type definition.
    #[serde(rename = "$value", default)]
    body: Vec<ComplexTypeBody>,
}

impl ComplexType {
    /// Retrieves the optional annotation element associated with the `ComplexType`.
    ///
    /// Complex types in XSD can have an optional annotation element that
    /// provides additional metadata or documentation for the type itself.
    /// This annotation can contain human-readable text or other elements
    /// for richer descriptions.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [Annotation].
    /// If exactly one annotation element is found, it is returned as an
    /// `Option<&Annotation>`.
    ///
    /// If no annotation element is present or there are multiple annotations
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Some(annotation)` if a single annotation element is found.
    /// * `None` if no annotation is present or there are multiple annotations.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ComplexTypeBody::Annotation)
    }

    /// Retrieves the optional `All` element associated with the `ComplexType`.
    ///
    /// Complex types in XSD can have an optional `All` element that specifies
    /// a restriction on the allowed child elements. This `All` element can
    /// contain references to other elements defined within the schema.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [All].
    /// If exactly one annotation element is found, it is returned as an
    /// `Option<&All>`.
    ///
    /// If no `All` element is present or there are multiple `All` elements
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Some(all)` if a single `All` element is found.
    /// * `None` if no `All` element is present or there are multiple `All` elements.
    pub fn all(&self) -> Option<&All> {
        element_from_body!(self, ComplexTypeBody::All)
    }

    /// Retrieves all `Assert` elements associated with the `ComplexType`.
    ///
    /// Complex types in XSD can have zero or more `Assert` elements. These
    /// elements define assertions that must hold true for an instance
    /// of the complex type. Assertions can be used for validation purposes.
    ///
    /// This method iterates through the `body` elements
    /// of the `ComplexType` and collect all elements of type
    /// [Assert]. It returns a vector containing references
    /// to all the `Assert` elements found within the complex type definition.
    ///
    /// If no `Assert` elements are present, the returned vector will be empty.
    ///
    /// # Returns
    ///
    /// * A vector containing references to all `Assert` elements within the complex type.
    pub fn asserts(&self) -> Vec<&Assert> {
        elements_from_body!(self, ComplexTypeBody::Assert)
    }

    /// Retrieves the optional `Sequence` element associated with the `ComplexType`.
    ///
    /// Complex types in XSD can have a single `Sequence` element that
    /// defines the order in which child elements can appear within an
    /// instance of the complex type. The `Sequence` element can contain
    /// references to other elements defined within the schema.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for an element of type [Sequence].
    /// If exactly one `Sequence` element is found, it is returned as an
    /// `Option<&Sequence>`.
    ///
    /// If no sequence element is present or there are multiple sequences
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// Returns
    /// * `Some(sequence)` if a single `Sequence` element is found.
    /// * `None` if no `Sequence` element is present or there are multiple `Sequence` elements.
    pub fn sequence(&self) -> Option<&Sequence> {
        element_from_body!(self, ComplexTypeBody::Sequence)
    }

    /// Retrieves all `Attribute` elements associated with the `ComplexType`.
    ///
    /// Complex types in XSD can have zero or more `Attribute` elements.
    /// These define attributes that can be associated with instances of the
    /// complex type. Attributes provide additional metadata or properties
    /// for the complex type.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and collects all elements of type [Attribute]. It returns
    /// a vector containing references to all the `Attribute` elements found
    /// within the complex type definition.
    ///
    /// If no `Attribute` elements are present, the returned vector will be empty.
    ///
    /// Returns:
    ///  * A vector containing references to all `Attribute` elements within the complex type
    pub fn attributes(&self) -> Vec<&Attribute> {
        elements_from_body!(self, ComplexTypeBody::Attribute)
    }

    /// Retrieves all `AttributeGroup` references associated with the `ComplexType`.
    ///
    /// Complex types in XSD can reference zero or more `AttributeGroup`
    /// definitions. These groups define collections of attributes that can be
    /// reused across multiple complex types.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and collects all elements of type [AttributeGroup].
    /// It returns a vector containing references to all the `AttributeGroup`
    /// references found within the complex type definition.
    ///
    /// If no `AttributeGroup` references are present, the returned vector will be empty.
    ///
    /// Returns:
    ///  * A vector containing references to all `AttributeGroup` references within the complex type.
    pub fn attribute_groups(&self) -> Vec<&AttributeGroup> {
        elements_from_body!(self, ComplexTypeBody::AttributeGroup)
    }

    /// Retrieves the optional `AnyAttribute` element associated with the `ComplexType`.
    ///
    /// Complex types can optionally have an `AnyAttribute` element. This element
    /// allows for any attribute from any namespace to be present on instances
    /// of the complex type.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [AnyAttribute].
    /// If exactly one anyAttribute element is found, it is returned as an
    /// `Option<&AnyAttribute>`.
    ///
    /// If no `AnyAttribute` element is present or there are multiple
    /// `AnyAttribute` elements (which is not valid according to the XSD schema),
    /// `None` is returned.
    ///
    /// Returns:
    ///  * `Some(any_attribute)` if a single `AnyAttribute` element is found.
    ///  * `None` if no `AnyAttribute` element is present or there are multiple `AnyAttribute` elements.
    pub fn any_attribute(&self) -> Option<&AnyAttribute> {
        element_from_body!(self, ComplexTypeBody::AnyAttribute)
    }

    /// Retrieves the optional `Group` element associated with the `ComplexType`.
    ///
    /// Complex types can have a single `Group` element that references a named
    /// group of elements defined elsewhere in the schema. This allows for
    /// reusability of element groups across different complex types.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [Group].
    /// If exactly one group element is found,it is returned as an `Option<&Group>`.
    ///
    /// If no `Group` element is present or there are multiple `Group` elements
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// Returns:
    ///  * `Some(group)` if a single `Group` element is found.
    ///  * `None` if no `Group` element is present or there are multiple `Group` elements.
    pub fn group(&self) -> Option<&Group> {
        element_from_body!(self, ComplexTypeBody::Group)
    }

    /// Retrieves the optional `ComplexContent` element associated with the `ComplexType`.
    ///
    /// Complex types can have a single `ComplexContent` element that defines
    /// the content model for the complex type by referencing another complex type.
    /// This allows for inheritance and extension of complex types.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [ComplexContent].
    /// If exactly one complexContent element is found, it is returned as an
    /// `Option<&ComplexContent>`.
    ///
    /// If no complexContent element is present or there are multiple
    /// complexContent elements (which is not valid according to the XSD schema),
    /// `None` is returned.
    ///
    /// Returns:
    ///  * `Some(complex_content)` if a single `ComplexContent` element is found.
    ///  * `None` if no `ComplexContent` element is present or there are multiple `ComplexContent` elements.
    pub fn complex_content(&self) -> Option<&ComplexContent> {
        element_from_body!(self, ComplexTypeBody::ComplexContent)
    }

    /// Retrieves the optional simpleContent element associated with the `ComplexType`.
    ///
    /// Complex types can have a single simpleContent` element that defines
    /// the content model for the complex type using a simple type. This allows
    /// for complex types to contain character data or other atomic values.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [SimpleContent].
    /// If exactly one `SimpleContent` element is found, it is returned as an
    /// `Option<&SimpleContent>`.
    ///
    /// If no simpleContent element is present or there are multiple
    /// simpleContent elements (which is not valid according to the XSD schema),
    /// `None` is returned.
    ///
    /// Returns:
    ///  * `Some(SimpleContent)` if a single simpleContent element is found.
    ///  * `None` if no simpleContent element is present or there are multiple simpleContent elements.
    pub fn simple_content(&self) -> Option<&SimpleContent> {
        element_from_body!(self, ComplexTypeBody::SimpleContent)
    }

    /// Retrieves the optional `choice` element associated with the `ComplexType`.
    ///
    /// Complex types can have a single `choice` element that defines a set of
    /// alternative elements. Only one of the elements within the `choice` can
    /// be present in an instance of the complex type.
    ///
    /// This method iterates through the `body` elements of the `ComplexType`
    /// and searches for elements of type [Choice].
    /// If exactly one `Choice` element is found, it is returned as an
    /// `Option<&Choice>`.
    ///
    /// If no `choice` element is present or there are multiple `choice` elements
    /// (which is not valid according to the XSD schema), `None` is returned.
    ///
    /// Returns:
    ///  * `Some(Choice)` if a single `choice` element is found.
    ///  * `None` if no `choice` element is present or there are multiple `choice` elements.
    pub fn choice(&self) -> Option<&Choice> {
        element_from_body!(self, ComplexTypeBody::Choice)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ComplexTypeBody {
    Annotation(Annotation),
    All(All),
    Assert(Assert),
    Sequence(Sequence),
    Attribute(Attribute),
    AttributeGroup(AttributeGroup),
    AnyAttribute(AnyAttribute),
    Group(Group),
    ComplexContent(ComplexContent),
    SimpleContent(SimpleContent),
    OpenContent(OpenContent),
    Choice(Choice),
}

/// Represents an `openContent` element within an XSD element declaration.
///
/// The `openContent` element specifies the allowed content for an element, similar to
/// complex type definitions. However, it is used within element declarations to provide
/// more flexibility in the allowed content model. This allows you to define what elements
/// can appear as children of the element.
///
/// Compared to `DefaultOpenContent` used for complex types, `OpenContent` can have a stricter
/// mode by disallowing elements from any namespace.
///
/// ```xsd
/// <openContent
///   id = ID
///   mode = (none | interleave | suffix) : interleave
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, any?)
/// </openContent>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OpenContent {
    /// Optional identifier for the `openContent` element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Open content mode for the complex type.
    ///
    /// The `@mode` attribute specifies how elements from outside the schema can be mixed
    /// with elements declared within the schema for this complex type. This is relevant when
    /// using wildcard elements (`Any`) within the `openContent` definition.
    ///
    /// The possible values for `@mode` are:
    ///
    /// * `Interleave`: Allows elements from any namespace to appear interleaved with elements
    ///   declared within the schema for this complex type. This means elements from any
    ///   namespace can be mixed freely with elements declared in the schema as children
    ///   of the complex type element.
    /// * `Suffix`: Allows elements from any namespace to appear only after elements declared
    ///   within the schema for this complex type. This enforces that elements declared in the
    ///   schema must appear first, followed by any additional elements from other namespaces.
    ///
    /// The choice of mode depends on the desired structure and validation for the complex type content.
    /// `Interleave` provides more flexibility for mixing elements, while `Suffix` ensures a
    /// specific order and stricter validation for elements declared in the schema.
    #[serde(rename = "@mode")]
    pub mode: Option<OpenContentMode>,
    /// Content allowed within the open content definition.
    ///
    /// The `body` vector can contain elements of type `Any` or `Annotation`. This allows you
    /// to specify wildcard elements using `Any` or provide additional comments or metadata
    /// through `Annotation` elements.
    #[serde(rename = "$value")]
    body: Vec<OpenContentBody>,
}

impl OpenContent {
    /// Extracts the optional annotation element from the `openContent` body.
    ///
    /// This method searches for an `Annotation` element within the `body` vector of the
    /// `OpenContent` struct. If an `Annotation` element is present, this method
    /// returns a reference to the contained `Annotation` struct. Otherwise, it returns `None`.
    ///
    /// This method is useful for retrieving any comments or metadata associated with the
    /// `openContent` definition.
    pub fn annotations(&self) -> Option<&Annotation> {
        element_from_body!(self, OpenContentBody::Annotation)
    }

    /// Extracts the optional "any" element from the `openContent` body.
    ///
    /// This method searches for an `Any` element within the `body` vector of the
    /// `openContent` struct. If an `Any` element is present, this method returns a
    /// reference to the contained `Any` struct. Otherwise, it returns `None`.
    ///
    /// An "any" element allows elements from any namespace to appear within the complex type
    /// content, potentially loosening validation constraints. This method is useful for
    /// accessing the wildcard element definition if present within the `openContent`.
    pub fn any(&self) -> Option<&Any> {
        element_from_body!(self, OpenContentBody::Any)
    }
}

/// Represents a simple content model for a complex type definition within an XSD schema.
///
/// A simple content model restricts the content of an element to a simple type. This means
/// the element can only contain character data (text) or a value from a predefined simple
/// type.
///
/// ```xsd
/// <simpleContent
///   id = ID
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (restriction | extension))
/// </simpleContent>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleContent {
    /// Optional identifier for the simple content.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Content definition for the simple content.
    ///
    /// The `body` vector can contain elements of type `Annotation`, `Restriction`, or `Extension`.
    /// Annotations provide comments or metadata. Restrictions can further constrain the allowed
    /// values for the simple content based on the underlying simple type. Extensions allow for
    /// adding custom elements within the simple content model.
    #[serde(rename = "$value", default)]
    body: Vec<ContentBody>,
}

impl SimpleContent {
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ContentBody::Annotation)
    }
}

/// Represents a complex content model for a complex type definition within an XSD schema.
///
/// A complex content model allows an element to inherit a content model from another complex
/// type while potentially adding restrictions or extensions. This provides a mechanism for
/// building complex types that reuse existing structures and customize them further.
///
/// ```xsd
/// <complexContent
///   id = ID
///   mixed = boolean
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (restriction | extension))
/// </complexContent>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexContent {
    /// Optional identifier for the complex content.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Mixed content model flag.
    ///
    /// The `@mixed` attribute specifies whether the complex content allows elements and character
    /// data (text) to be mixed within its content. If set to `true`, both elements and text
    /// can appear as children of the element using this complex type.
    #[serde(rename = "@mixed")]
    pub mixed: Option<bool>,
    /// Content definition for the complex content.
    ///
    /// The `body` vector can contain elements of type `Annotation`, `Restriction`, or `Extension`.
    /// Annotations provide comments or metadata. Restrictions can further constrain the inherited
    /// content model. Extensions allow for adding custom elements within the complex content.
    #[serde(rename = "$value")]
    body: Vec<ContentBody>,
}

impl ComplexContent {
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ContentBody::Annotation)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ContentBody {
    Annotation(Annotation),
    Restriction(Restriction),
    Extension(Extension),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct AppInfo {
    #[serde(rename = "@source")]
    source: Option<AnyURI>,
    // #[serde(rename = "$text")]
    // pub body: Option<Vec<String>>,
}

/// Represents an annotation element within an XSD schema.
///
/// Annotations provide comments or metadata for various elements within the schema, including
/// complex types, elements, attributes, groups, and even particle definitions. They are used
/// to document the schema or provide additional information that is not part of the strict
/// validation rules of the XSD.
///
/// ```xsd
/// <annotation
///   id = ID
///   {any attributes with non-schema namespace . . .}>
///   Content: (appinfo | documentation)*
/// </annotation>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Annotation {
    /// Optional namespace URI for the annotation element.
    ///
    /// This attribute allows you to specify a namespace for the annotation, which can be useful
    /// if you are using custom annotation elements from a specific vocabulary.
    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,
    /// Content of the annotation element.
    ///
    /// The content of the annotation can be any text or XML elements depending on the specific
    /// annotation type used within the `body` vector. Common annotation types include
    /// `appinfo` for application-specific information and `documentation` for human-readable
    /// comments.
    #[serde(rename = "$value", default)]
    body: Vec<AnnotationBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum AnnotationBody {
    #[serde(rename = "appinfo")]
    AppInfo(AppInfo),
    Documentation(Documentation),
}

/// Represents an extension element within an XSD complex type definition.
///
/// The `extension` element allows you to add custom elements or attributes to an existing
/// complex type definition. This provides a mechanism for extending the content model of a
/// complex type without modifying the base type itself. This can be useful for adding
/// application-specific elements or attributes that are not part of the core schema definition.
///
/// ```xsd
/// <extension
///   base = QName
///   id = ID
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, openContent?, ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?), assert*))
/// </extension>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extension {
    /// Optional identifier for the extension element.
    #[serde(rename = "@id")]
    pub id: Option<String>,
    /// Base type for the extension.
    ///
    /// The `@base` attribute specifies the complex type that this extension is based on. The
    /// extension inherits the content model from the base type but can add additional elements
    /// or attributes.
    #[serde(rename = "@base")]
    pub base: QName,
    /// Content allowed within the extension element.
    ///
    /// The `body` vector can contain various elements that define the additional content allowed
    /// by the extension. This can include elements like `All`, `Sequence`, `Choice`, `Attribute`,
    /// `AnyAttribute`, and others, allowing you to specify the structure and allowed components
    /// within the extension.
    #[serde(rename = "$value", default)]
    body: Vec<ExtensionBody>,
}

impl Extension {
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, ExtensionBody::Annotation)
    }

    pub fn open_content(&self) -> Option<&OpenContent> {
        element_from_body!(self, ExtensionBody::OpenContent)
    }

    pub fn asserts(&self) -> Vec<&Assert> {
        elements_from_body!(self, ExtensionBody::Assert)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ExtensionBody {
    All(All),
    Assert(Assert),
    Group(Group),
    Attribute(Attribute),
    AnyAttribute(AnyAttribute),
    AttributeGroup(AttributeGroup),
    Sequence(Sequence),
    Choice(Choice),
    Annotation(Annotation),
    OpenContent(OpenContent),
}

/// Represents an XML Schema documentation element.
///
/// A documentation element in XSD allows you to provide human-readable
/// comments or explanations for various schema components. These comments
/// are not processed by the schema validator but can be used for
/// documentation purposes.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Documentation {
    /// Optional source of the documentation.
    ///
    /// The `@source` attribute is an optional attribute on the `xs:documentation`
    /// element. It can be used to specify the source of the documentation,
    /// such as a reference to an external document.
    #[serde(rename = "@source")]
    pub source: Option<String>,
    /// Optional language for the documentation.
    ///
    /// The `@xml:lang` attribute is an optional attribute on the `xs:documentation`
    /// element. It can be used to specify the language of the documentation
    /// for better human readability.
    #[serde(rename = "@lang")]
    pub xml_lang: Option<String>,
    /// Content of the documentation.
    ///
    /// The body of the `xs:documentation` element can contain text content
    /// representing the actual documentation for the schema component. This
    /// can be plain text, formatted markup (depending on the schema processor),
    /// or references to external documentation resources.
    #[serde(rename = "$value", default)]
    pub body: Vec<String>,
}

/// Represents a `unique` element within an XSD schema.
///
/// The `unique` element defines a unique constraint that ensures no element instance
/// within the scope of the schema document can have identical values for the specified
/// set of fields. This helps maintain data integrity by preventing duplicate entries based
/// on the chosen fields.
///
/// ```xsd
/// <unique
///   id = ID
///   name = NCName
///   ref = QName
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?, (selector, field+)?)
/// </unique>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Unique {
    /// Optional identifier for the unique constraint.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Name of the unique constraint.
    #[serde(rename = "@name")]
    pub name: NCName,
    /// Reference to an existing unique constraint (optional).
    ///
    /// The `@ref` attribute allows you to reference a pre-defined unique constraint by its name
    /// (qualified name) instead of providing inline definitions for selector and field. This
    /// promotes code reuse and avoids redundancy in the schema.
    #[serde(rename = "@ref")]
    pub r#ref: Option<QName>,
    /// Content definition for the unique constraint (if inline definition is used).
    ///
    /// The `body` vector can contain elements of type `Annotation`, `Selector`, or `Field`.
    /// Annotations provide comments or metadata. The `selector` element specifies the path
    /// to the element(s) for which uniqueness is enforced. The `field` element identifies
    /// the specific field(s) within the selected element(s) that must be unique. This is used
    /// for inline definitions only, and is mutually exclusive with the `@ref` attribute.
    #[serde(rename = "$value", default)]
    body: Vec<UniqueBody>,
}

impl Unique {
    /// Extracts the annotation element from the unique body.
    ///
    /// This method iterates through the body of the unique and
    /// collects all elements of type [Annotation]. If found, it returns
    /// a reference to that annotation element, otherwise it returns `None`.
    ///
    /// Annotations can be used within attribute declarations to provide
    /// documentation or other descriptive information about the unique.
    pub fn annotation(&self) -> Option<&Annotation> {
        element_from_body!(self, UniqueBody::Annotation)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum UniqueBody {
    Annotation(Annotation),
    Selector(Selector),
    Field(Field),
}

/// Represents a `selector` element within an XSD schema, used with unique constraints.
///
/// The `selector` element specifies the XPath expression that identifies the target element(s)
/// to which a unique constraint applies. This element is used within the definition of a
/// `unique` element to define the scope of the uniqueness constraint.
///
/// ```xsd
/// <selector
///   id = ID
///   xpath = a subset of XPath expression, see below
///   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </selector>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Selector {
    /// Optional identifier for the selector element.
    #[serde(rename = "@id")]
    id: Option<String>,
    /// XPath expression to identify target elements.
    ///
    /// The `@xpath` attribute is mandatory and specifies the XPath expression that selects the
    /// element(s) for which the unique constraint applies. This expression must evaluate to
    /// one or more element nodes within the schema document.
    #[serde(rename = "@xpath")]
    xpath: String,
    /// Optional default namespace for the XPath expression.
    ///
    /// The `@xpathDefaultNamespace` attribute allows you to specify a default namespace for the
    /// prefixes used within the XPath expression. This can help simplify the expression and avoid
    /// the need to explicitly declare prefixes for all namespaces used.
    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<AnyURI>,
    /// Optional annotation element for comments or metadata.
    ///
    /// The `body` field can optionally contain an `Annotation` element. This can be used to
    /// provide additional information or documentation about the selector and its purpose within
    /// the unique constraint definition.
    body: Option<Annotation>,
}

impl Selector {
    /// Extracts the optional `xs:annotation` element from the selector.
    ///
    /// This method retrieves the optional `Annotation` element stored within the `body` field
    /// of the `Selector` struct. If an annotation element exists, it returns a reference to that
    /// element, otherwise it returns `None`.
    ///
    /// Annotations can be used within selector elements to provide comments or additional
    /// descriptive information about the specific XPath expression used to target elements for
    /// a unique constraint. This can be helpful in understanding the rationale behind the chosen
    /// selection criteria.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Key {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "$value", default)]
    body: Vec<KeyBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum KeyBody {
    Annotation(Annotation),
    Selector(Selector),
    Field(Field),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Keyref {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@name")]
    name: NCName,
    #[serde(rename = "@refer")]
    refer: QName,
    #[serde(rename = "$value")]
    body: Vec<KeyrefBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum KeyrefBody {
    Annotation(Annotation),
    Selector(Selector),
    Field(Field),
}

/// Represents a `field` element within an XSD schema, used with unique constraints.
///
/// The `field` element specifies an XPath expression that identifies the specific field(s)
/// within the target element(s) for which uniqueness must be enforced. This element is used
/// within the definition of a `unique` element to define which data points within the selected
/// elements need to be unique to ensure data integrity.
///
/// ```xsd
/// <field
///   id = ID
///   xpath = a subset of XPath expression, see below
///   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
///   {any attributes with non-schema namespace . . .}>
///   Content: (annotation?)
/// </field>
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Field {
    /// Optional identifier for the field element.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// XPath expression to identify target field(s).
    ///
    /// The `@xpath` attribute is mandatory and specifies the XPath expression that selects the
    /// specific field(s) within the target element(s) for which uniqueness applies. This expression
    /// must evaluate to one or more attribute or element nodes within the selected element(s).
    #[serde(rename = "@xpath")]
    pub xpath: String,
    /// Optional default namespace for the XPath expression.
    ///
    /// The `@xpathDefaultNamespace` attribute allows you to specify a default namespace for the
    /// prefixes used within the XPath expression. This can help simplify the expression and avoid
    /// the need to explicitly declare prefixes for all namespaces used.
    #[serde(rename = "@xpathDefaultNamespace")]
    /// Optional annotation element for comments or metadata.
    ///
    /// The `body` field can optionally contain an `Annotation` element. This can be used to
    /// provide additional information or documentation about the field and its purpose within
    /// the unique constraint definition.
    pub xpath_default_namespace: Option<AnyURI>,
    body: Option<Annotation>,
}

impl Field {
    /// Extracts the optional `xs:annotation` element from the field.
    ///
    /// This method retrieves the optional `Annotation` element stored within the `body` field
    /// of the `Field` struct. If an annotation element exists, it returns a reference to that
    /// element, otherwise it returns `None`.
    ///
    /// Annotations can be used within field elements to provide comments or additional
    /// descriptive information about the specific XPath expression used to target fields within
    /// a unique constraint. This can be helpful in understanding the rationale behind the chosen
    /// field selection for enforcing uniqueness.
    pub fn annotation(&self) -> Option<&Annotation> {
        self.body.as_ref()
    }
}

/// Represents an XML Schema assert element.
///
/// An assert element in XSD allows you to define assertions within the schema.
/// Assertions are conditions or expressions that must be evaluated as true
/// for an instance document to be considered valid. However, support for
/// assertions may vary depending on the schema validator used.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Assert {
    /// Optional identifier for the assert element.
    ///
    /// The `@id` attribute is an optional attribute on the `xs:assert` element.
    /// It allows you to specify a unique identifier for the assertion.
    #[serde(rename = "@id")]
    pub id: Option<ID>,
    /// Optional test expression for the assertion.
    ///
    /// The `@test` attribute is an optional attribute on the `xs:assert` element.
    /// It specifies the XPath expression that must evaluate to true for the
    /// assertion to pass.
    #[serde(rename = "@test")]
    pub test: Option<String>,
    /// Optional annotation associated with the assert element.
    ///
    /// The body of the `xs:assert` element can optionally contain an
    /// annotation element that provides comments or explanations for the
    /// assertion.
    #[serde(rename = "$value")]
    pub annotation: Option<Annotation>,
}
