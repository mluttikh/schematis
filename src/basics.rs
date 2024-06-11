/// Represents a string value that conforms to the anyURI data type in XSD.
/// The `anyURI` data type is a built-in XSD type used to specify a Uniform
/// Resource Identifier (URI). URIs can be used to reference various kinds of
/// resources, including web pages, files, images, and more.
pub type AnyURI = String;

/// Represents a string value conforming to the ID data type in XSD. The ID
/// data type is used for unique identifiers within an XML document based on
/// an XML Schema (XSD) definition.
///
/// An ID value must:
///  - Start with a letter or underscore (_).
///  - Contain letters, digits, underscores, hyphens (-), periods (.), or
///    colons (:) following the first character.
///
/// ID values are required to be unique within the scope of the document
/// referencing the XSD. This ensures that each element or attribute with
/// an ID can be uniquely identified.
///
/// This type is typically used within XSD to define attributes or elements
/// that act as unique identifiers within the schema itself or within the
/// XML documents that conform to the schema.
pub type ID = String;

/// Represents a string value conforming to the NCName data type in XSD.
/// NCName (Name without Colons) is a built-in XSD type used for XML names
/// that cannot contain colons (":"). This is useful for element names,
/// attribute names, and other identifiers within an XML document.
///
/// An NCName must start with a letter or underscore (_), and can contain
/// letters, digits, underscores, hyphens (-), and periods (.) afterwards.
///
/// This type is typically used within XSD to define valid names for
/// elements, attributes, and other constructs within the schema itself.
pub type NCName = String;

/// Represents a qualified name as defined in XML Schemas (XSD).
///
/// A qualified name consists of two parts:
///  - Prefix: An optional prefix that identifies a namespace.
///  - Local name: The name of the element, attribute, type, etc. within
///    that namespace.
///
/// This type is typically used within XSD to represent references to elements,
/// attributes, complex types, simple types, and other constructs defined
/// within the schema or imported from other schemas.
pub type QName = String;

/// Represents a string value conforming to the `xsd:token` data type in XSD.
///
/// The `xsd:token` data type is a built-in XSD type used for string values
/// that may contain most characters allowed in XML. However, it applies
/// specific whitespace handling:
///
///  - All occurrences of carriage returns, line feeds, and tabs are replaced
///    with a single space character.
///  - Consecutive spaces are collapsed into a single space character.
///  - Leading and trailing spaces are removed.
///
/// This processing is equivalent to the processing of non-CDATA attribute
/// values in XML 1.0.
///
/// This type is commonly used for element names, attribute names, and other
/// identifiers within an XSD schema or XML documents conforming to the
/// schema. However, unlike the `NCName` type, `token` allows characters
/// other than letters, digits, underscores, hyphens, and periods.
///
/// Be aware that the name `token` can be misleading, as it might imply
/// a single character or a short string. In reality, `token` can contain
/// various characters after whitespace processing.
pub type Token = String;
