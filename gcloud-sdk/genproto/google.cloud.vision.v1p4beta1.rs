/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// A 3D position in the image, used primarily for Face detection landmarks.
/// A valid Position must have both x and y coordinates.
/// The position coordinates are in the same scale as the original image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
    /// Z coordinate (or depth).
    #[prost(float, tag = "3")]
    pub z: f32,
}
/// Parameters for a celebrity recognition request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceRecognitionParams {
    /// The resource names for one or more
    /// \[CelebritySet][google.cloud.vision.v1p4beta1.CelebritySet\]s. A celebrity
    /// set is preloaded and can be specified as "builtin/default". If this is
    /// specified, the algorithm will try to match the faces detected in the input
    /// image to the Celebrities in the CelebritySets.
    #[prost(string, repeated, tag = "1")]
    pub celebrity_set: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Celebrity is a group of Faces with an identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Celebrity {
    /// The resource name of the preloaded Celebrity. Has the format
    /// `builtin/{mid}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The Celebrity's display name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The Celebrity's description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Information about a face's identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceRecognitionResult {
    /// The \[Celebrity][google.cloud.vision.v1p4beta1.Celebrity\] that this face was
    /// matched to.
    #[prost(message, optional, tag = "1")]
    pub celebrity: ::core::option::Option<Celebrity>,
    /// Recognition confidence. Range [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// A Product contains ReferenceImages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// The resource name of the product.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    ///
    /// This field is ignored when creating a product.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-provided name for this Product. Must not be empty. Must be at most
    /// 4096 characters long.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// User-provided metadata to be stored with this product. Must be at most 4096
    /// characters long.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The category for the product identified by the reference image.
    /// This should be either "homegoods-v2", "apparel-v2", or "toys-v2". The
    /// legacy categories "homegoods", "apparel", and "toys" are still supported,
    /// but these should not be used for new products.
    #[prost(string, tag = "4")]
    pub product_category: ::prost::alloc::string::String,
    /// Key-value pairs that can be attached to a product. At query time,
    /// constraints can be specified based on the product_labels.
    ///
    /// Note that integer values can be provided as strings, e.g. "1199". Only
    /// strings with integer values can match a range-based restriction which is
    /// to be supported soon.
    ///
    /// Multiple values can be assigned to the same key. One product may have up to
    /// 500 product_labels.
    ///
    /// Notice that the total number of distinct product_labels over all products
    /// in one ProductSet cannot exceed 1M, otherwise the product search pipeline
    /// will refuse to work for that ProductSet.
    #[prost(message, repeated, tag = "5")]
    pub product_labels: ::prost::alloc::vec::Vec<product::KeyValue>,
}
/// Nested message and enum types in `Product`.
pub mod product {
    /// A product label represented as a key-value pair.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValue {
        /// The key of the label attached to the product. Cannot be empty and cannot
        /// exceed 128 bytes.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The value of the label attached to the product. Cannot be empty and
        /// cannot exceed 128 bytes.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
}
/// A ProductSet contains Products. A ProductSet can contain a maximum of 1
/// million reference images. If the limit is exceeded, periodic indexing will
/// fail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSet {
    /// The resource name of the ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`.
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-provided name for this ProductSet. Must not be empty. Must be at
    /// most 4096 characters long.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The time at which this ProductSet was last indexed. Query
    /// results will reflect all updates before this time. If this ProductSet has
    /// never been indexed, this timestamp is the default value
    /// "1970-01-01T00:00:00Z".
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(message, optional, tag = "3")]
    pub index_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If there was an error with indexing the product set, the field
    /// is populated.
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(message, optional, tag = "4")]
    pub index_error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A `ReferenceImage` represents a product image and its associated metadata,
/// such as bounding boxes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceImage {
    /// The resource name of the reference image.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.
    ///
    /// This field is ignored when creating a reference image.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Google Cloud Storage URI of the reference image.
    ///
    /// The URI must start with `gs://`.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Optional. Bounding polygons around the areas of interest in the reference
    /// image. If this field is empty, the system will try to detect regions of
    /// interest. At most 10 bounding polygons will be used.
    ///
    /// The provided shape is converted into a non-rotated rectangle. Once
    /// converted, the small edge of the rectangle must be greater than or equal
    /// to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5
    /// is not).
    #[prost(message, repeated, tag = "3")]
    pub bounding_polys: ::prost::alloc::vec::Vec<BoundingPoly>,
}
/// Request message for the `CreateProduct` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    /// Required. The project in which the Product should be created.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The product to create.
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
    /// A user-supplied resource id for this Product. If set, the server will
    /// attempt to use this value as the resource id. If it is already in use, an
    /// error is returned with code ALREADY_EXISTS. Must be at most 128 characters
    /// long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub product_id: ::prost::alloc::string::String,
}
/// Request message for the `ListProducts` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    /// Required. The project OR ProductSet from which Products should be listed.
    ///
    /// Format:
    /// `projects/PROJECT_ID/locations/LOC_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListProducts` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    /// List of products.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `GetProduct` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    /// Required. Resource name of the Product to get.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `UpdateProduct` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductRequest {
    /// Required. The Product resource which replaces the one on the server.
    /// product.name is immutable.
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
    /// The \[FieldMask][google.protobuf.FieldMask\] that specifies which fields
    /// to update.
    /// If update_mask isn't specified, all mutable fields are to be updated.
    /// Valid mask paths include `product_labels`, `display_name`, and
    /// `description`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the `DeleteProduct` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductRequest {
    /// Required. Resource name of product to delete.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `CreateProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductSetRequest {
    /// Required. The project in which the ProductSet should be created.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ProductSet to create.
    #[prost(message, optional, tag = "2")]
    pub product_set: ::core::option::Option<ProductSet>,
    /// A user-supplied resource id for this ProductSet. If set, the server will
    /// attempt to use this value as the resource id. If it is already in use, an
    /// error is returned with code ALREADY_EXISTS. Must be at most 128 characters
    /// long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub product_set_id: ::prost::alloc::string::String,
}
/// Request message for the `ListProductSets` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductSetsRequest {
    /// Required. The project from which ProductSets should be listed.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListProductSets` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductSetsResponse {
    /// List of ProductSets.
    #[prost(message, repeated, tag = "1")]
    pub product_sets: ::prost::alloc::vec::Vec<ProductSet>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `GetProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductSetRequest {
    /// Required. Resource name of the ProductSet to get.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `UpdateProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductSetRequest {
    /// Required. The ProductSet resource which replaces the one on the server.
    #[prost(message, optional, tag = "1")]
    pub product_set: ::core::option::Option<ProductSet>,
    /// The \[FieldMask][google.protobuf.FieldMask\] that specifies which fields to
    /// update.
    /// If update_mask isn't specified, all mutable fields are to be updated.
    /// Valid mask path is `display_name`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the `DeleteProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductSetRequest {
    /// Required. Resource name of the ProductSet to delete.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `CreateReferenceImage` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReferenceImageRequest {
    /// Required. Resource name of the product in which to create the reference
    /// image.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The reference image to create.
    /// If an image ID is specified, it is ignored.
    #[prost(message, optional, tag = "2")]
    pub reference_image: ::core::option::Option<ReferenceImage>,
    /// A user-supplied resource id for the ReferenceImage to be added. If set,
    /// the server will attempt to use this value as the resource id. If it is
    /// already in use, an error is returned with code ALREADY_EXISTS. Must be at
    /// most 128 characters long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub reference_image_id: ::prost::alloc::string::String,
}
/// Request message for the `ListReferenceImages` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferenceImagesRequest {
    /// Required. Resource name of the product containing the reference images.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results to be returned. This is the value
    /// of `nextPageToken` returned in a previous reference image list request.
    ///
    /// Defaults to the first page if not specified.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListReferenceImages` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferenceImagesResponse {
    /// The list of reference images.
    #[prost(message, repeated, tag = "1")]
    pub reference_images: ::prost::alloc::vec::Vec<ReferenceImage>,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `GetReferenceImage` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferenceImageRequest {
    /// Required. The resource name of the ReferenceImage to get.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `DeleteReferenceImage` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReferenceImageRequest {
    /// Required. The resource name of the reference image to delete.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `AddProductToProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProductToProductSetRequest {
    /// Required. The resource name for the ProductSet to modify.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name for the Product to be added to this ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "2")]
    pub product: ::prost::alloc::string::String,
}
/// Request message for the `RemoveProductFromProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProductFromProductSetRequest {
    /// Required. The resource name for the ProductSet to modify.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name for the Product to be removed from this
    /// ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "2")]
    pub product: ::prost::alloc::string::String,
}
/// Request message for the `ListProductsInProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsInProductSetRequest {
    /// Required. The ProductSet resource for which to retrieve Products.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListProductsInProductSet` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsInProductSetResponse {
    /// The list of Products.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location for a csv file which preserves a list of
/// ImportProductSetRequests in each line.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsGcsSource {
    /// The Google Cloud Storage URI of the input csv file.
    ///
    /// The URI must start with `gs://`.
    ///
    /// The format of the input csv file should be one image per line.
    /// In each line, there are 8 columns.
    ///
    /// 1.  image-uri
    /// 2.  image-id
    /// 3.  product-set-id
    /// 4.  product-id
    /// 5.  product-category
    /// 6.  product-display-name
    /// 7.  labels
    /// 8.  bounding-poly
    ///
    /// The `image-uri`, `product-set-id`, `product-id`, and `product-category`
    /// columns are required. All other columns are optional.
    ///
    /// If the `ProductSet` or `Product` specified by the `product-set-id` and
    /// `product-id` values does not exist, then the system will create a new
    /// `ProductSet` or `Product` for the image. In this case, the
    /// `product-display-name` column refers to
    /// \[display_name][google.cloud.vision.v1p4beta1.Product.display_name\], the
    /// `product-category` column refers to
    /// \[product_category][google.cloud.vision.v1p4beta1.Product.product_category\],
    /// and the `labels` column refers to
    /// \[product_labels][google.cloud.vision.v1p4beta1.Product.product_labels\].
    ///
    /// The `image-id` column is optional but must be unique if provided. If it is
    /// empty, the system will automatically assign a unique id to the image.
    ///
    /// The `product-display-name` column is optional. If it is empty, the system
    /// sets the \[display_name][google.cloud.vision.v1p4beta1.Product.display_name\]
    /// field for the product to a space (" "). You can update the `display_name`
    /// later by using the API.
    ///
    /// If a `Product` with the specified `product-id` already exists, then the
    /// system ignores the `product-display-name`, `product-category`, and `labels`
    /// columns.
    ///
    /// The `labels` column (optional) is a line containing a list of
    /// comma-separated key-value pairs, in the following format:
    ///
    ///      "key_1=value_1,key_2=value_2,...,key_n=value_n"
    ///
    /// The `bounding-poly` column (optional) identifies one region of
    /// interest from the image in the same manner as `CreateReferenceImage`. If
    /// you do not specify the `bounding-poly` column, then the system will try to
    /// detect regions of interest automatically.
    ///
    /// At most one `bounding-poly` column is allowed per line. If the image
    /// contains multiple regions of interest, add a line to the CSV file that
    /// includes the same product information, and the `bounding-poly` values for
    /// each region of interest.
    ///
    /// The `bounding-poly` column must contain an even number of comma-separated
    /// numbers, in the format "p1_x,p1_y,p2_x,p2_y,...,pn_x,pn_y". Use
    /// non-negative integers for absolute bounding polygons, and float values
    /// in [0, 1] for normalized bounding polygons.
    ///
    /// The system will resize the image if the image resolution is too
    /// large to process (larger than 20MP).
    #[prost(string, tag = "1")]
    pub csv_file_uri: ::prost::alloc::string::String,
}
/// The input content for the `ImportProductSets` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsInputConfig {
    /// The source of the input.
    #[prost(oneof = "import_product_sets_input_config::Source", tags = "1")]
    pub source: ::core::option::Option<import_product_sets_input_config::Source>,
}
/// Nested message and enum types in `ImportProductSetsInputConfig`.
pub mod import_product_sets_input_config {
    /// The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location for a csv file which preserves a list
        /// of ImportProductSetRequests in each line.
        #[prost(message, tag = "1")]
        GcsSource(super::ImportProductSetsGcsSource),
    }
}
/// Request message for the `ImportProductSets` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsRequest {
    /// Required. The project in which the ProductSets should be imported.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The input content for the list of requests.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::core::option::Option<ImportProductSetsInputConfig>,
}
/// Response message for the `ImportProductSets` method.
///
/// This message is returned by the
/// \[google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation\]
/// method in the returned
/// \[google.longrunning.Operation.response][google.longrunning.Operation.response\]
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsResponse {
    /// The list of reference_images that are imported successfully.
    #[prost(message, repeated, tag = "1")]
    pub reference_images: ::prost::alloc::vec::Vec<ReferenceImage>,
    /// The rpc status for each ImportProductSet request, including both successes
    /// and errors.
    ///
    /// The number of statuses here matches the number of lines in the csv file,
    /// and statuses\[i\] stores the success or failure status of processing the i-th
    /// line of the csv, starting from line 0.
    #[prost(message, repeated, tag = "2")]
    pub statuses: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Metadata for the batch operations such as the current state.
///
/// This is included in the `metadata` field of the `Operation` returned by the
/// `GetOperation` call of the `google::longrunning::Operations` service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperationMetadata {
    /// The current state of the batch operation.
    #[prost(enumeration = "batch_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The time when the batch request was submitted to the server.
    #[prost(message, optional, tag = "2")]
    pub submit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the batch request is finished and
    /// \[google.longrunning.Operation.done][google.longrunning.Operation.done\] is
    /// set to true.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `BatchOperationMetadata`.
pub mod batch_operation_metadata {
    /// Enumerates the possible states that the batch request can be in.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Invalid.
        Unspecified = 0,
        /// Request is actively being processed.
        Processing = 1,
        /// The request is done and at least one item has been successfully
        /// processed.
        Successful = 2,
        /// The request is done and no item has been successfully processed.
        Failed = 3,
        /// The request is done after the longrunning.Operations.CancelOperation has
        /// been called by the user.  Any records that were processed before the
        /// cancel command are output as specified in the request.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Processing => "PROCESSING",
                State::Successful => "SUCCESSFUL",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROCESSING" => Some(Self::Processing),
                "SUCCESSFUL" => Some(Self::Successful),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// Config to control which ProductSet contains the Products to be deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSetPurgeConfig {
    /// The ProductSet that contains the Products to delete. If a Product is a
    /// member of product_set_id in addition to other ProductSets, the Product will
    /// still be deleted.
    #[prost(string, tag = "1")]
    pub product_set_id: ::prost::alloc::string::String,
}
/// Request message for the `PurgeProducts` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeProductsRequest {
    /// Required. The project and location in which the Products should be deleted.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The default value is false. Override this value to true to actually perform
    /// the purge.
    #[prost(bool, tag = "4")]
    pub force: bool,
    /// The Products to delete.
    #[prost(oneof = "purge_products_request::Target", tags = "2, 3")]
    pub target: ::core::option::Option<purge_products_request::Target>,
}
/// Nested message and enum types in `PurgeProductsRequest`.
pub mod purge_products_request {
    /// The Products to delete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Specify which ProductSet contains the Products to be deleted.
        #[prost(message, tag = "2")]
        ProductSetPurgeConfig(super::ProductSetPurgeConfig),
        /// If delete_orphan_products is true, all Products that are not in any
        /// ProductSet will be deleted.
        #[prost(bool, tag = "3")]
        DeleteOrphanProducts(bool),
    }
}
/// Generated client implementations.
pub mod product_search_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages Products and ProductSets of reference images for use in product
    /// search. It uses the following resource model:
    ///
    /// - The API has a collection of
    /// [ProductSet][google.cloud.vision.v1p4beta1.ProductSet] resources, named
    /// `projects/*/locations/*/productSets/*`, which acts as a way to put different
    /// products into groups to limit identification.
    ///
    /// In parallel,
    ///
    /// - The API has a collection of
    /// [Product][google.cloud.vision.v1p4beta1.Product] resources, named
    ///   `projects/*/locations/*/products/*`
    ///
    /// - Each [Product][google.cloud.vision.v1p4beta1.Product] has a collection of
    /// [ReferenceImage][google.cloud.vision.v1p4beta1.ReferenceImage] resources,
    /// named
    ///   `projects/*/locations/*/products/*/referenceImages/*`
    #[derive(Debug, Clone)]
    pub struct ProductSearchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProductSearchClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProductSearchClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProductSearchClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ProductSearchClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Creates and returns a new ProductSet resource.
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if display_name is missing, or is longer than
        ///   4096 characters.
        pub async fn create_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/CreateProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists ProductSets in an unspecified order.
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if page_size is greater than 100, or less
        ///   than 1.
        pub async fn list_product_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductSetsRequest>,
        ) -> Result<tonic::Response<super::ListProductSetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/ListProductSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information associated with a ProductSet.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the ProductSet does not exist.
        pub async fn get_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/GetProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Makes changes to a ProductSet resource.
        /// Only display_name can be updated currently.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the ProductSet does not exist.
        /// * Returns INVALID_ARGUMENT if display_name is present in update_mask but
        ///   missing from the request or longer than 4096 characters.
        pub async fn update_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/UpdateProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes a ProductSet. Products and ReferenceImages in the
        /// ProductSet are not deleted.
        ///
        /// The actual image files are not deleted from Google Cloud Storage.
        pub async fn delete_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/DeleteProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates and returns a new product resource.
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096
        ///   characters.
        /// * Returns INVALID_ARGUMENT if description is longer than 4096 characters.
        /// * Returns INVALID_ARGUMENT if product_category is missing or invalid.
        pub async fn create_product(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/CreateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists products in an unspecified order.
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> Result<tonic::Response<super::ListProductsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/ListProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information associated with a Product.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the Product does not exist.
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/GetProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Makes changes to a Product resource.
        /// Only the `display_name`, `description`, and `labels` fields can be updated
        /// right now.
        ///
        /// If labels are updated, the change will not be reflected in queries until
        /// the next index time.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the Product does not exist.
        /// * Returns INVALID_ARGUMENT if display_name is present in update_mask but is
        ///   missing from the request or longer than 4096 characters.
        /// * Returns INVALID_ARGUMENT if description is present in update_mask but is
        ///   longer than 4096 characters.
        /// * Returns INVALID_ARGUMENT if product_category is present in update_mask.
        pub async fn update_product(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/UpdateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes a product and its reference images.
        ///
        /// Metadata of the product and all its images will be deleted right away, but
        /// search queries against ProductSets containing the product may still work
        /// until all related caches are refreshed.
        pub async fn delete_product(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProductRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/DeleteProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates and returns a new ReferenceImage resource.
        ///
        /// The `bounding_poly` field is optional. If `bounding_poly` is not specified,
        /// the system will try to detect regions of interest in the image that are
        /// compatible with the product_category on the parent product. If it is
        /// specified, detection is ALWAYS skipped. The system converts polygons into
        /// non-rotated rectangles.
        ///
        /// Note that the pipeline will resize the image if the image resolution is too
        /// large to process (above 50MP).
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096
        ///   characters.
        /// * Returns INVALID_ARGUMENT if the product does not exist.
        /// * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing
        ///   compatible with the parent product's product_category is detected.
        /// * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons.
        pub async fn create_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/CreateReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes a reference image.
        ///
        /// The image metadata will be deleted right away, but search queries
        /// against ProductSets containing the image may still work until all related
        /// caches are refreshed.
        ///
        /// The actual image files are not deleted from Google Cloud Storage.
        pub async fn delete_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReferenceImageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/DeleteReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists reference images.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the parent product does not exist.
        /// * Returns INVALID_ARGUMENT if the page_size is greater than 100, or less
        ///   than 1.
        pub async fn list_reference_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReferenceImagesRequest>,
        ) -> Result<tonic::Response<super::ListReferenceImagesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/ListReferenceImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information associated with a ReferenceImage.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the specified image does not exist.
        pub async fn get_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/GetReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds a Product to the specified ProductSet. If the Product is already
        /// present, no change is made.
        ///
        /// One Product can be added to at most 100 ProductSets.
        ///
        /// Possible errors:
        ///
        /// * Returns NOT_FOUND if the Product or the ProductSet doesn't exist.
        pub async fn add_product_to_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProductToProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/AddProductToProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a Product from the specified ProductSet.
        pub async fn remove_product_from_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProductFromProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/RemoveProductFromProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the Products in a ProductSet, in an unspecified order. If the
        /// ProductSet does not exist, the products field of the response will be
        /// empty.
        ///
        /// Possible errors:
        ///
        /// * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.
        pub async fn list_products_in_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsInProductSetRequest>,
        ) -> Result<
            tonic::Response<super::ListProductsInProductSetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/ListProductsInProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Asynchronous API that imports a list of reference images to specified
        /// product sets based on a list of image information.
        ///
        /// The [google.longrunning.Operation][google.longrunning.Operation] API can be
        /// used to keep track of the progress and results of the request.
        /// `Operation.metadata` contains `BatchOperationMetadata`. (progress)
        /// `Operation.response` contains `ImportProductSetsResponse`. (results)
        ///
        /// The input source of this method is a csv file on Google Cloud Storage.
        /// For the format of the csv file please see
        /// [ImportProductSetsGcsSource.csv_file_uri][google.cloud.vision.v1p4beta1.ImportProductSetsGcsSource.csv_file_uri].
        pub async fn import_product_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportProductSetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/ImportProductSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Asynchronous API to delete all Products in a ProductSet or all Products
        /// that are in no ProductSet.
        ///
        /// If a Product is a member of the specified ProductSet in addition to other
        /// ProductSets, the Product will still be deleted.
        ///
        /// It is recommended to not delete the specified ProductSet until after this
        /// operation has completed. It is also recommended to not add any of the
        /// Products involved in the batch delete to a new ProductSet while this
        /// operation is running because those Products may still end up deleted.
        ///
        /// It's not possible to undo the PurgeProducts operation. Therefore, it is
        /// recommended to keep the csv files used in ImportProductSets (if that was
        /// how you originally built the Product Set) before starting PurgeProducts, in
        /// case you need to re-import the data after deletion.
        ///
        /// If the plan is to purge all of the Products from a ProductSet and then
        /// re-use the empty ProductSet to re-import new Products into the empty
        /// ProductSet, you must wait until the PurgeProducts operation has finished
        /// for that ProductSet.
        ///
        /// The [google.longrunning.Operation][google.longrunning.Operation] API can be
        /// used to keep track of the progress and results of the request.
        /// `Operation.metadata` contains `BatchOperationMetadata`. (progress)
        pub async fn purge_products(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeProductsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ProductSearch/PurgeProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Parameters for a product search request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSearchParams {
    /// The bounding polygon around the area of interest in the image.
    /// If it is not specified, system discretion will be applied.
    #[prost(message, optional, tag = "9")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// The resource name of a
    /// \[ProductSet][google.cloud.vision.v1p4beta1.ProductSet\] to be searched for
    /// similar images.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`.
    #[prost(string, tag = "6")]
    pub product_set: ::prost::alloc::string::String,
    /// The list of product categories to search in. Currently, we only consider
    /// the first category, and either "homegoods-v2", "apparel-v2", "toys-v2",
    /// "packagedgoods-v1", or "general-v1" should be specified. The legacy
    /// categories "homegoods", "apparel", and "toys" are still supported but will
    /// be deprecated. For new products, please use "homegoods-v2", "apparel-v2",
    /// or "toys-v2" for better product search accuracy. It is recommended to
    /// migrate existing products to these categories as well.
    #[prost(string, repeated, tag = "7")]
    pub product_categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The filtering expression. This can be used to restrict search results based
    /// on Product labels. We currently support an AND of OR of key-value
    /// expressions, where each expression within an OR must have the same key. An
    /// '=' should be used to connect the key and value.
    ///
    /// For example, "(color = red OR color = blue) AND brand = Google" is
    /// acceptable, but "(color = red OR brand = Google)" is not acceptable.
    /// "color: red" is not acceptable because it uses a ':' instead of an '='.
    #[prost(string, tag = "8")]
    pub filter: ::prost::alloc::string::String,
}
/// Results for a product search request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSearchResults {
    /// Timestamp of the index which provided these results. Products added to the
    /// product set and products removed from the product set after this time are
    /// not reflected in the current results.
    #[prost(message, optional, tag = "2")]
    pub index_time: ::core::option::Option<::prost_types::Timestamp>,
    /// List of results, one for each product match.
    #[prost(message, repeated, tag = "5")]
    pub results: ::prost::alloc::vec::Vec<product_search_results::Result>,
    /// List of results grouped by products detected in the query image. Each entry
    /// corresponds to one bounding polygon in the query image, and contains the
    /// matching products specific to that region. There may be duplicate product
    /// matches in the union of all the per-product results.
    #[prost(message, repeated, tag = "6")]
    pub product_grouped_results: ::prost::alloc::vec::Vec<
        product_search_results::GroupedResult,
    >,
}
/// Nested message and enum types in `ProductSearchResults`.
pub mod product_search_results {
    /// Information about a product.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        /// The Product.
        #[prost(message, optional, tag = "1")]
        pub product: ::core::option::Option<super::Product>,
        /// A confidence level on the match, ranging from 0 (no confidence) to
        /// 1 (full confidence).
        #[prost(float, tag = "2")]
        pub score: f32,
        /// The resource name of the image from the product that is the closest match
        /// to the query.
        #[prost(string, tag = "3")]
        pub image: ::prost::alloc::string::String,
    }
    /// Prediction for what the object in the bounding box is.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectAnnotation {
        /// Object ID that should align with EntityAnnotation mid.
        #[prost(string, tag = "1")]
        pub mid: ::prost::alloc::string::String,
        /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
        /// information, see
        /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
        #[prost(string, tag = "2")]
        pub language_code: ::prost::alloc::string::String,
        /// Object name, expressed in its `language_code` language.
        #[prost(string, tag = "3")]
        pub name: ::prost::alloc::string::String,
        /// Score of the result. Range [0, 1].
        #[prost(float, tag = "4")]
        pub score: f32,
    }
    /// Information about the products similar to a single product in a query
    /// image.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupedResult {
        /// The bounding polygon around the product detected in the query image.
        #[prost(message, optional, tag = "1")]
        pub bounding_poly: ::core::option::Option<super::BoundingPoly>,
        /// List of results, one for each product match.
        #[prost(message, repeated, tag = "2")]
        pub results: ::prost::alloc::vec::Vec<Result>,
        /// List of generic predictions for the object in the bounding box.
        #[prost(message, repeated, tag = "3")]
        pub object_annotations: ::prost::alloc::vec::Vec<ObjectAnnotation>,
    }
}
/// TextAnnotation contains a structured representation of OCR extracted text.
/// The hierarchy of an OCR extracted text structure is like this:
///      TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol
/// Each structural component, starting from Page, may further have their own
/// properties. Properties describe detected languages, breaks etc.. Please refer
/// to the
/// \[TextAnnotation.TextProperty][google.cloud.vision.v1p4beta1.TextAnnotation.TextProperty\]
/// message definition below for more detail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAnnotation {
    /// List of pages detected by OCR.
    #[prost(message, repeated, tag = "1")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    /// UTF-8 text detected on the pages.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TextAnnotation`.
pub mod text_annotation {
    /// Detected language for a structural component.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedLanguage {
        /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
        /// information, see
        /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
        #[prost(string, tag = "1")]
        pub language_code: ::prost::alloc::string::String,
        /// Confidence of detected language. Range [0, 1].
        #[prost(float, tag = "2")]
        pub confidence: f32,
    }
    /// Detected start or end of a structural component.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedBreak {
        /// Detected break type.
        #[prost(enumeration = "detected_break::BreakType", tag = "1")]
        pub r#type: i32,
        /// True if break prepends the element.
        #[prost(bool, tag = "2")]
        pub is_prefix: bool,
    }
    /// Nested message and enum types in `DetectedBreak`.
    pub mod detected_break {
        /// Enum to denote the type of break found. New line, space etc.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum BreakType {
            /// Unknown break label type.
            Unknown = 0,
            /// Regular space.
            Space = 1,
            /// Sure space (very wide).
            SureSpace = 2,
            /// Line-wrapping break.
            EolSureSpace = 3,
            /// End-line hyphen that is not present in text; does not co-occur with
            /// `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`.
            Hyphen = 4,
            /// Line break that ends a paragraph.
            LineBreak = 5,
        }
        impl BreakType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    BreakType::Unknown => "UNKNOWN",
                    BreakType::Space => "SPACE",
                    BreakType::SureSpace => "SURE_SPACE",
                    BreakType::EolSureSpace => "EOL_SURE_SPACE",
                    BreakType::Hyphen => "HYPHEN",
                    BreakType::LineBreak => "LINE_BREAK",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "SPACE" => Some(Self::Space),
                    "SURE_SPACE" => Some(Self::SureSpace),
                    "EOL_SURE_SPACE" => Some(Self::EolSureSpace),
                    "HYPHEN" => Some(Self::Hyphen),
                    "LINE_BREAK" => Some(Self::LineBreak),
                    _ => None,
                }
            }
        }
    }
    /// Additional information detected on the structural component.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextProperty {
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "1")]
        pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        /// Detected start or end of a text segment.
        #[prost(message, optional, tag = "2")]
        pub detected_break: ::core::option::Option<DetectedBreak>,
    }
}
/// Detected page from OCR.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// Additional information detected on the page.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// Page width. For PDFs the unit is points. For images (including
    /// TIFFs) the unit is pixels.
    #[prost(int32, tag = "2")]
    pub width: i32,
    /// Page height. For PDFs the unit is points. For images (including
    /// TIFFs) the unit is pixels.
    #[prost(int32, tag = "3")]
    pub height: i32,
    /// List of blocks of text, images etc on this page.
    #[prost(message, repeated, tag = "4")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    /// Confidence of the OCR results on the page. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Logical element on the page.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Additional information detected for the block.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the block.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///
    /// * when the text is horizontal it might look like:
    ///
    ///          0----1
    ///          |    |
    ///          3----2
    ///
    /// * when it's rotated 180 degrees around the top-left corner it becomes:
    ///
    ///          2----3
    ///          |    |
    ///          1----0
    ///
    ///    and the vertex order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of paragraphs in this block (if this blocks is of type text).
    #[prost(message, repeated, tag = "3")]
    pub paragraphs: ::prost::alloc::vec::Vec<Paragraph>,
    /// Detected block type (text, image etc) for this block.
    #[prost(enumeration = "block::BlockType", tag = "4")]
    pub block_type: i32,
    /// Confidence of the OCR results on the block. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Nested message and enum types in `Block`.
pub mod block {
    /// Type of a block (text, image etc) as identified by OCR.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BlockType {
        /// Unknown block type.
        Unknown = 0,
        /// Regular text block.
        Text = 1,
        /// Table block.
        Table = 2,
        /// Image block.
        Picture = 3,
        /// Horizontal/vertical line box.
        Ruler = 4,
        /// Barcode block.
        Barcode = 5,
    }
    impl BlockType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BlockType::Unknown => "UNKNOWN",
                BlockType::Text => "TEXT",
                BlockType::Table => "TABLE",
                BlockType::Picture => "PICTURE",
                BlockType::Ruler => "RULER",
                BlockType::Barcode => "BARCODE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "TEXT" => Some(Self::Text),
                "TABLE" => Some(Self::Table),
                "PICTURE" => Some(Self::Picture),
                "RULER" => Some(Self::Ruler),
                "BARCODE" => Some(Self::Barcode),
                _ => None,
            }
        }
    }
}
/// Structural unit of text representing a number of words in certain order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paragraph {
    /// Additional information detected for the paragraph.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the paragraph.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///    * when the text is horizontal it might look like:
    ///       0----1
    ///       |    |
    ///       3----2
    ///    * when it's rotated 180 degrees around the top-left corner it becomes:
    ///       2----3
    ///       |    |
    ///       1----0
    ///    and the vertex order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of all words in this paragraph.
    #[prost(message, repeated, tag = "3")]
    pub words: ::prost::alloc::vec::Vec<Word>,
    /// Confidence of the OCR results for the paragraph. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A word representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Word {
    /// Additional information detected for the word.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the word.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///    * when the text is horizontal it might look like:
    ///       0----1
    ///       |    |
    ///       3----2
    ///    * when it's rotated 180 degrees around the top-left corner it becomes:
    ///       2----3
    ///       |    |
    ///       1----0
    ///    and the vertex order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of symbols in the word.
    /// The order of the symbols follows the natural reading order.
    #[prost(message, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<Symbol>,
    /// Confidence of the OCR results for the word. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A single symbol representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symbol {
    /// Additional information detected for the symbol.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the symbol.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///    * when the text is horizontal it might look like:
    ///       0----1
    ///       |    |
    ///       3----2
    ///    * when it's rotated 180 degrees around the top-left corner it becomes:
    ///       2----3
    ///       |    |
    ///       1----0
    ///    and the vertex order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// The actual UTF-8 representation of the symbol.
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// Confidence of the OCR results for the symbol. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Relevant information for the image from the Internet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetection {
    /// Deduced entities from similar images on the Internet.
    #[prost(message, repeated, tag = "1")]
    pub web_entities: ::prost::alloc::vec::Vec<web_detection::WebEntity>,
    /// Fully matching images from the Internet.
    /// Can include resized copies of the query image.
    #[prost(message, repeated, tag = "2")]
    pub full_matching_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// Partial matching images from the Internet.
    /// Those images are similar enough to share some key-point features. For
    /// example an original image will likely have partial matching for its crops.
    #[prost(message, repeated, tag = "3")]
    pub partial_matching_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// Web pages containing the matching images from the Internet.
    #[prost(message, repeated, tag = "4")]
    pub pages_with_matching_images: ::prost::alloc::vec::Vec<web_detection::WebPage>,
    /// The visually similar image results.
    #[prost(message, repeated, tag = "6")]
    pub visually_similar_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// The service's best guess as to the topic of the request image.
    /// Inferred from similar images on the open web.
    #[prost(message, repeated, tag = "8")]
    pub best_guess_labels: ::prost::alloc::vec::Vec<web_detection::WebLabel>,
}
/// Nested message and enum types in `WebDetection`.
pub mod web_detection {
    /// Entity deduced from similar images on the Internet.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebEntity {
        /// Opaque entity ID.
        #[prost(string, tag = "1")]
        pub entity_id: ::prost::alloc::string::String,
        /// Overall relevancy score for the entity.
        /// Not normalized and not comparable across different image queries.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Canonical description of the entity, in English.
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
    }
    /// Metadata for online images.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebImage {
        /// The result image URL.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        /// (Deprecated) Overall relevancy score for the image.
        #[prost(float, tag = "2")]
        pub score: f32,
    }
    /// Metadata for web pages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebPage {
        /// The result web page URL.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        /// (Deprecated) Overall relevancy score for the web page.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Title for the web page, may contain HTML markups.
        #[prost(string, tag = "3")]
        pub page_title: ::prost::alloc::string::String,
        /// Fully matching images on the page.
        /// Can include resized copies of the query image.
        #[prost(message, repeated, tag = "4")]
        pub full_matching_images: ::prost::alloc::vec::Vec<WebImage>,
        /// Partial matching images on the page.
        /// Those images are similar enough to share some key-point features. For
        /// example an original image will likely have partial matching for its
        /// crops.
        #[prost(message, repeated, tag = "5")]
        pub partial_matching_images: ::prost::alloc::vec::Vec<WebImage>,
    }
    /// Label to provide extra metadata for the web detection.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebLabel {
        /// Label for extra metadata.
        #[prost(string, tag = "1")]
        pub label: ::prost::alloc::string::String,
        /// The BCP-47 language code for `label`, such as "en-US" or "sr-Latn".
        /// For more information, see
        /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
        #[prost(string, tag = "2")]
        pub language_code: ::prost::alloc::string::String,
    }
}
/// The type of Google Cloud Vision API detection to perform, and the maximum
/// number of results to return for that type. Multiple `Feature` objects can
/// be specified in the `features` list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// The feature type.
    #[prost(enumeration = "feature::Type", tag = "1")]
    pub r#type: i32,
    /// Maximum number of results of this type. Does not apply to
    /// `TEXT_DETECTION`, `DOCUMENT_TEXT_DETECTION`, or `CROP_HINTS`.
    #[prost(int32, tag = "2")]
    pub max_results: i32,
    /// Model to use for the feature.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest". `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` also
    /// support "builtin/weekly" for the bleeding edge release updated weekly.
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Feature`.
pub mod feature {
    /// Type of Google Cloud Vision API feature to be extracted.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified feature type.
        Unspecified = 0,
        /// Run face detection.
        FaceDetection = 1,
        /// Run landmark detection.
        LandmarkDetection = 2,
        /// Run logo detection.
        LogoDetection = 3,
        /// Run label detection.
        LabelDetection = 4,
        /// Run text detection / optical character recognition (OCR). Text detection
        /// is optimized for areas of text within a larger image; if the image is
        /// a document, use `DOCUMENT_TEXT_DETECTION` instead.
        TextDetection = 5,
        /// Run dense text document OCR. Takes precedence when both
        /// `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` are present.
        DocumentTextDetection = 11,
        /// Run Safe Search to detect potentially unsafe
        /// or undesirable content.
        SafeSearchDetection = 6,
        /// Compute a set of image properties, such as the
        /// image's dominant colors.
        ImageProperties = 7,
        /// Run crop hints.
        CropHints = 9,
        /// Run web detection.
        WebDetection = 10,
        /// Run Product Search.
        ProductSearch = 12,
        /// Run localizer for object detection.
        ObjectLocalization = 19,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::FaceDetection => "FACE_DETECTION",
                Type::LandmarkDetection => "LANDMARK_DETECTION",
                Type::LogoDetection => "LOGO_DETECTION",
                Type::LabelDetection => "LABEL_DETECTION",
                Type::TextDetection => "TEXT_DETECTION",
                Type::DocumentTextDetection => "DOCUMENT_TEXT_DETECTION",
                Type::SafeSearchDetection => "SAFE_SEARCH_DETECTION",
                Type::ImageProperties => "IMAGE_PROPERTIES",
                Type::CropHints => "CROP_HINTS",
                Type::WebDetection => "WEB_DETECTION",
                Type::ProductSearch => "PRODUCT_SEARCH",
                Type::ObjectLocalization => "OBJECT_LOCALIZATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FACE_DETECTION" => Some(Self::FaceDetection),
                "LANDMARK_DETECTION" => Some(Self::LandmarkDetection),
                "LOGO_DETECTION" => Some(Self::LogoDetection),
                "LABEL_DETECTION" => Some(Self::LabelDetection),
                "TEXT_DETECTION" => Some(Self::TextDetection),
                "DOCUMENT_TEXT_DETECTION" => Some(Self::DocumentTextDetection),
                "SAFE_SEARCH_DETECTION" => Some(Self::SafeSearchDetection),
                "IMAGE_PROPERTIES" => Some(Self::ImageProperties),
                "CROP_HINTS" => Some(Self::CropHints),
                "WEB_DETECTION" => Some(Self::WebDetection),
                "PRODUCT_SEARCH" => Some(Self::ProductSearch),
                "OBJECT_LOCALIZATION" => Some(Self::ObjectLocalization),
                _ => None,
            }
        }
    }
}
/// External image source (Google Cloud Storage or web URL image location).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSource {
    /// **Use `image_uri` instead.**
    ///
    /// The Google Cloud Storage  URI of the form
    /// `gs://bucket_name/object_name`. Object versioning is not supported. See
    /// [Google Cloud Storage Request
    /// URIs](<https://cloud.google.com/storage/docs/reference-uris>) for more info.
    #[prost(string, tag = "1")]
    pub gcs_image_uri: ::prost::alloc::string::String,
    /// The URI of the source image. Can be either:
    ///
    /// 1. A Google Cloud Storage URI of the form
    ///     `gs://bucket_name/object_name`. Object versioning is not supported. See
    ///     [Google Cloud Storage Request
    ///     URIs](<https://cloud.google.com/storage/docs/reference-uris>) for more
    ///     info.
    ///
    /// 2. A publicly-accessible image HTTP/HTTPS URL. When fetching images from
    ///     HTTP/HTTPS URLs, Google cannot guarantee that the request will be
    ///     completed. Your request may fail if the specified host denies the
    ///     request (e.g. due to request throttling or DOS prevention), or if Google
    ///     throttles requests to the site for abuse prevention. You should not
    ///     depend on externally-hosted images for production applications.
    ///
    /// When both `gcs_image_uri` and `image_uri` are specified, `image_uri` takes
    /// precedence.
    #[prost(string, tag = "2")]
    pub image_uri: ::prost::alloc::string::String,
}
/// Client image to perform Google Cloud Vision API tasks over.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Image content, represented as a stream of bytes.
    /// Note: As with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// Google Cloud Storage image location, or publicly-accessible image
    /// URL. If both `content` and `source` are provided for an image, `content`
    /// takes precedence and is used to perform the image annotation request.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<ImageSource>,
}
/// A face annotation object contains the results of face detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// The bounding polygon around the face. The coordinates of the bounding box
    /// are in the original image's scale.
    /// The bounding box is computed to "frame" the face in accordance with human
    /// expectations. It is based on the landmarker results.
    /// Note that one or more x and/or y coordinates may not be generated in the
    /// `BoundingPoly` (the polygon will be unbounded) if only a partial face
    /// appears in the image to be annotated.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// The `fd_bounding_poly` bounding polygon is tighter than the
    /// `boundingPoly`, and encloses only the skin part of the face. Typically, it
    /// is used to eliminate the face from any image analysis that detects the
    /// "amount of skin" visible in an image. It is not based on the
    /// landmarker results, only on the initial face detection, hence
    /// the <code>fd</code> (face detection) prefix.
    #[prost(message, optional, tag = "2")]
    pub fd_bounding_poly: ::core::option::Option<BoundingPoly>,
    /// Detected face landmarks.
    #[prost(message, repeated, tag = "3")]
    pub landmarks: ::prost::alloc::vec::Vec<face_annotation::Landmark>,
    /// Roll angle, which indicates the amount of clockwise/anti-clockwise rotation
    /// of the face relative to the image vertical about the axis perpendicular to
    /// the face. Range \[-180,180\].
    #[prost(float, tag = "4")]
    pub roll_angle: f32,
    /// Yaw angle, which indicates the leftward/rightward angle that the face is
    /// pointing relative to the vertical plane perpendicular to the image. Range
    /// \[-180,180\].
    #[prost(float, tag = "5")]
    pub pan_angle: f32,
    /// Pitch angle, which indicates the upwards/downwards angle that the face is
    /// pointing relative to the image's horizontal plane. Range \[-180,180\].
    #[prost(float, tag = "6")]
    pub tilt_angle: f32,
    /// Detection confidence. Range [0, 1].
    #[prost(float, tag = "7")]
    pub detection_confidence: f32,
    /// Face landmarking confidence. Range [0, 1].
    #[prost(float, tag = "8")]
    pub landmarking_confidence: f32,
    /// Joy likelihood.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub joy_likelihood: i32,
    /// Sorrow likelihood.
    #[prost(enumeration = "Likelihood", tag = "10")]
    pub sorrow_likelihood: i32,
    /// Anger likelihood.
    #[prost(enumeration = "Likelihood", tag = "11")]
    pub anger_likelihood: i32,
    /// Surprise likelihood.
    #[prost(enumeration = "Likelihood", tag = "12")]
    pub surprise_likelihood: i32,
    /// Under-exposed likelihood.
    #[prost(enumeration = "Likelihood", tag = "13")]
    pub under_exposed_likelihood: i32,
    /// Blurred likelihood.
    #[prost(enumeration = "Likelihood", tag = "14")]
    pub blurred_likelihood: i32,
    /// Headwear likelihood.
    #[prost(enumeration = "Likelihood", tag = "15")]
    pub headwear_likelihood: i32,
    /// Additional recognition information. Only computed if
    /// image_context.face_recognition_params is provided, **and** a match is found
    /// to a \[Celebrity][google.cloud.vision.v1p4beta1.Celebrity\] in the input
    /// \[CelebritySet][google.cloud.vision.v1p4beta1.CelebritySet\]. This field is
    /// sorted in order of decreasing confidence values.
    #[prost(message, repeated, tag = "16")]
    pub recognition_result: ::prost::alloc::vec::Vec<FaceRecognitionResult>,
}
/// Nested message and enum types in `FaceAnnotation`.
pub mod face_annotation {
    /// A face-specific landmark (for example, a face feature).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Landmark {
        /// Face landmark type.
        #[prost(enumeration = "landmark::Type", tag = "3")]
        pub r#type: i32,
        /// Face landmark position.
        #[prost(message, optional, tag = "4")]
        pub position: ::core::option::Option<super::Position>,
    }
    /// Nested message and enum types in `Landmark`.
    pub mod landmark {
        /// Face landmark (feature) type.
        /// Left and right are defined from the vantage of the viewer of the image
        /// without considering mirror projections typical of photos. So, `LEFT_EYE`,
        /// typically, is the person's right eye.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Type {
            /// Unknown face landmark detected. Should not be filled.
            UnknownLandmark = 0,
            /// Left eye.
            LeftEye = 1,
            /// Right eye.
            RightEye = 2,
            /// Left of left eyebrow.
            LeftOfLeftEyebrow = 3,
            /// Right of left eyebrow.
            RightOfLeftEyebrow = 4,
            /// Left of right eyebrow.
            LeftOfRightEyebrow = 5,
            /// Right of right eyebrow.
            RightOfRightEyebrow = 6,
            /// Midpoint between eyes.
            MidpointBetweenEyes = 7,
            /// Nose tip.
            NoseTip = 8,
            /// Upper lip.
            UpperLip = 9,
            /// Lower lip.
            LowerLip = 10,
            /// Mouth left.
            MouthLeft = 11,
            /// Mouth right.
            MouthRight = 12,
            /// Mouth center.
            MouthCenter = 13,
            /// Nose, bottom right.
            NoseBottomRight = 14,
            /// Nose, bottom left.
            NoseBottomLeft = 15,
            /// Nose, bottom center.
            NoseBottomCenter = 16,
            /// Left eye, top boundary.
            LeftEyeTopBoundary = 17,
            /// Left eye, right corner.
            LeftEyeRightCorner = 18,
            /// Left eye, bottom boundary.
            LeftEyeBottomBoundary = 19,
            /// Left eye, left corner.
            LeftEyeLeftCorner = 20,
            /// Right eye, top boundary.
            RightEyeTopBoundary = 21,
            /// Right eye, right corner.
            RightEyeRightCorner = 22,
            /// Right eye, bottom boundary.
            RightEyeBottomBoundary = 23,
            /// Right eye, left corner.
            RightEyeLeftCorner = 24,
            /// Left eyebrow, upper midpoint.
            LeftEyebrowUpperMidpoint = 25,
            /// Right eyebrow, upper midpoint.
            RightEyebrowUpperMidpoint = 26,
            /// Left ear tragion.
            LeftEarTragion = 27,
            /// Right ear tragion.
            RightEarTragion = 28,
            /// Left eye pupil.
            LeftEyePupil = 29,
            /// Right eye pupil.
            RightEyePupil = 30,
            /// Forehead glabella.
            ForeheadGlabella = 31,
            /// Chin gnathion.
            ChinGnathion = 32,
            /// Chin left gonion.
            ChinLeftGonion = 33,
            /// Chin right gonion.
            ChinRightGonion = 34,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::UnknownLandmark => "UNKNOWN_LANDMARK",
                    Type::LeftEye => "LEFT_EYE",
                    Type::RightEye => "RIGHT_EYE",
                    Type::LeftOfLeftEyebrow => "LEFT_OF_LEFT_EYEBROW",
                    Type::RightOfLeftEyebrow => "RIGHT_OF_LEFT_EYEBROW",
                    Type::LeftOfRightEyebrow => "LEFT_OF_RIGHT_EYEBROW",
                    Type::RightOfRightEyebrow => "RIGHT_OF_RIGHT_EYEBROW",
                    Type::MidpointBetweenEyes => "MIDPOINT_BETWEEN_EYES",
                    Type::NoseTip => "NOSE_TIP",
                    Type::UpperLip => "UPPER_LIP",
                    Type::LowerLip => "LOWER_LIP",
                    Type::MouthLeft => "MOUTH_LEFT",
                    Type::MouthRight => "MOUTH_RIGHT",
                    Type::MouthCenter => "MOUTH_CENTER",
                    Type::NoseBottomRight => "NOSE_BOTTOM_RIGHT",
                    Type::NoseBottomLeft => "NOSE_BOTTOM_LEFT",
                    Type::NoseBottomCenter => "NOSE_BOTTOM_CENTER",
                    Type::LeftEyeTopBoundary => "LEFT_EYE_TOP_BOUNDARY",
                    Type::LeftEyeRightCorner => "LEFT_EYE_RIGHT_CORNER",
                    Type::LeftEyeBottomBoundary => "LEFT_EYE_BOTTOM_BOUNDARY",
                    Type::LeftEyeLeftCorner => "LEFT_EYE_LEFT_CORNER",
                    Type::RightEyeTopBoundary => "RIGHT_EYE_TOP_BOUNDARY",
                    Type::RightEyeRightCorner => "RIGHT_EYE_RIGHT_CORNER",
                    Type::RightEyeBottomBoundary => "RIGHT_EYE_BOTTOM_BOUNDARY",
                    Type::RightEyeLeftCorner => "RIGHT_EYE_LEFT_CORNER",
                    Type::LeftEyebrowUpperMidpoint => "LEFT_EYEBROW_UPPER_MIDPOINT",
                    Type::RightEyebrowUpperMidpoint => "RIGHT_EYEBROW_UPPER_MIDPOINT",
                    Type::LeftEarTragion => "LEFT_EAR_TRAGION",
                    Type::RightEarTragion => "RIGHT_EAR_TRAGION",
                    Type::LeftEyePupil => "LEFT_EYE_PUPIL",
                    Type::RightEyePupil => "RIGHT_EYE_PUPIL",
                    Type::ForeheadGlabella => "FOREHEAD_GLABELLA",
                    Type::ChinGnathion => "CHIN_GNATHION",
                    Type::ChinLeftGonion => "CHIN_LEFT_GONION",
                    Type::ChinRightGonion => "CHIN_RIGHT_GONION",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN_LANDMARK" => Some(Self::UnknownLandmark),
                    "LEFT_EYE" => Some(Self::LeftEye),
                    "RIGHT_EYE" => Some(Self::RightEye),
                    "LEFT_OF_LEFT_EYEBROW" => Some(Self::LeftOfLeftEyebrow),
                    "RIGHT_OF_LEFT_EYEBROW" => Some(Self::RightOfLeftEyebrow),
                    "LEFT_OF_RIGHT_EYEBROW" => Some(Self::LeftOfRightEyebrow),
                    "RIGHT_OF_RIGHT_EYEBROW" => Some(Self::RightOfRightEyebrow),
                    "MIDPOINT_BETWEEN_EYES" => Some(Self::MidpointBetweenEyes),
                    "NOSE_TIP" => Some(Self::NoseTip),
                    "UPPER_LIP" => Some(Self::UpperLip),
                    "LOWER_LIP" => Some(Self::LowerLip),
                    "MOUTH_LEFT" => Some(Self::MouthLeft),
                    "MOUTH_RIGHT" => Some(Self::MouthRight),
                    "MOUTH_CENTER" => Some(Self::MouthCenter),
                    "NOSE_BOTTOM_RIGHT" => Some(Self::NoseBottomRight),
                    "NOSE_BOTTOM_LEFT" => Some(Self::NoseBottomLeft),
                    "NOSE_BOTTOM_CENTER" => Some(Self::NoseBottomCenter),
                    "LEFT_EYE_TOP_BOUNDARY" => Some(Self::LeftEyeTopBoundary),
                    "LEFT_EYE_RIGHT_CORNER" => Some(Self::LeftEyeRightCorner),
                    "LEFT_EYE_BOTTOM_BOUNDARY" => Some(Self::LeftEyeBottomBoundary),
                    "LEFT_EYE_LEFT_CORNER" => Some(Self::LeftEyeLeftCorner),
                    "RIGHT_EYE_TOP_BOUNDARY" => Some(Self::RightEyeTopBoundary),
                    "RIGHT_EYE_RIGHT_CORNER" => Some(Self::RightEyeRightCorner),
                    "RIGHT_EYE_BOTTOM_BOUNDARY" => Some(Self::RightEyeBottomBoundary),
                    "RIGHT_EYE_LEFT_CORNER" => Some(Self::RightEyeLeftCorner),
                    "LEFT_EYEBROW_UPPER_MIDPOINT" => Some(Self::LeftEyebrowUpperMidpoint),
                    "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                        Some(Self::RightEyebrowUpperMidpoint)
                    }
                    "LEFT_EAR_TRAGION" => Some(Self::LeftEarTragion),
                    "RIGHT_EAR_TRAGION" => Some(Self::RightEarTragion),
                    "LEFT_EYE_PUPIL" => Some(Self::LeftEyePupil),
                    "RIGHT_EYE_PUPIL" => Some(Self::RightEyePupil),
                    "FOREHEAD_GLABELLA" => Some(Self::ForeheadGlabella),
                    "CHIN_GNATHION" => Some(Self::ChinGnathion),
                    "CHIN_LEFT_GONION" => Some(Self::ChinLeftGonion),
                    "CHIN_RIGHT_GONION" => Some(Self::ChinRightGonion),
                    _ => None,
                }
            }
        }
    }
}
/// Detected entity location information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// lat/long location coordinates.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// A `Property` consists of a user-supplied name/value pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Name of the property.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the property.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Value of numeric properties.
    #[prost(uint64, tag = "3")]
    pub uint64_value: u64,
}
/// Set of detected entity features.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityAnnotation {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag = "1")]
    pub mid: ::prost::alloc::string::String,
    /// The language code for the locale in which the entity textual
    /// `description` is expressed.
    #[prost(string, tag = "2")]
    pub locale: ::prost::alloc::string::String,
    /// Entity textual description, expressed in its `locale` language.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Overall score of the result. Range [0, 1].
    #[prost(float, tag = "4")]
    pub score: f32,
    /// **Deprecated. Use `score` instead.**
    /// The accuracy of the entity detection in an image.
    /// For example, for an image in which the "Eiffel Tower" entity is detected,
    /// this field represents the confidence that there is a tower in the query
    /// image. Range [0, 1].
    #[deprecated]
    #[prost(float, tag = "5")]
    pub confidence: f32,
    /// The relevancy of the ICA (Image Content Annotation) label to the
    /// image. For example, the relevancy of "tower" is likely higher to an image
    /// containing the detected "Eiffel Tower" than to an image containing a
    /// detected distant towering building, even though the confidence that
    /// there is a tower in each image may be the same. Range [0, 1].
    #[prost(float, tag = "6")]
    pub topicality: f32,
    /// Image region to which this entity belongs. Not produced
    /// for `LABEL_DETECTION` features.
    #[prost(message, optional, tag = "7")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// The location information for the detected entity. Multiple
    /// `LocationInfo` elements can be present because one location may
    /// indicate the location of the scene in the image, and another location
    /// may indicate the location of the place where the image was taken.
    /// Location information is usually present for landmarks.
    #[prost(message, repeated, tag = "8")]
    pub locations: ::prost::alloc::vec::Vec<LocationInfo>,
    /// Some entities may have optional user-supplied `Property` (name/value)
    /// fields, such a score or string that qualifies the entity.
    #[prost(message, repeated, tag = "9")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
/// Set of detected objects with bounding boxes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedObjectAnnotation {
    /// Object ID that should align with EntityAnnotation mid.
    #[prost(string, tag = "1")]
    pub mid: ::prost::alloc::string::String,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Object name, expressed in its `language_code` language.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Score of the result. Range [0, 1].
    #[prost(float, tag = "4")]
    pub score: f32,
    /// Image region to which this object belongs. This must be populated.
    #[prost(message, optional, tag = "5")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
}
/// Set of features pertaining to the image, computed by computer vision
/// methods over safe-search verticals (for example, adult, spoof, medical,
/// violence).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeSearchAnnotation {
    /// Represents the adult content likelihood for the image. Adult content may
    /// contain elements such as nudity, pornographic images or cartoons, or
    /// sexual activities.
    #[prost(enumeration = "Likelihood", tag = "1")]
    pub adult: i32,
    /// Spoof likelihood. The likelihood that an modification
    /// was made to the image's canonical version to make it appear
    /// funny or offensive.
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub spoof: i32,
    /// Likelihood that this is a medical image.
    #[prost(enumeration = "Likelihood", tag = "3")]
    pub medical: i32,
    /// Likelihood that this image contains violent content.
    #[prost(enumeration = "Likelihood", tag = "4")]
    pub violence: i32,
    /// Likelihood that the request image contains racy content. Racy content may
    /// include (but is not limited to) skimpy or sheer clothing, strategically
    /// covered nudity, lewd or provocative poses, or close-ups of sensitive
    /// body areas.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub racy: i32,
}
/// Rectangle determined by min and max `LatLng` pairs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLongRect {
    /// Min lat/long pair.
    #[prost(message, optional, tag = "1")]
    pub min_lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Max lat/long pair.
    #[prost(message, optional, tag = "2")]
    pub max_lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// Color information consists of RGB channels, score, and the fraction of
/// the image that the color occupies in the image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorInfo {
    /// RGB components of the color.
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::super::r#type::Color>,
    /// Image-specific score for this color. Value in range [0, 1].
    #[prost(float, tag = "2")]
    pub score: f32,
    /// The fraction of pixels the color occupies in the image.
    /// Value in range [0, 1].
    #[prost(float, tag = "3")]
    pub pixel_fraction: f32,
}
/// Set of dominant colors and their corresponding scores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DominantColorsAnnotation {
    /// RGB color values with their score and pixel fraction.
    #[prost(message, repeated, tag = "1")]
    pub colors: ::prost::alloc::vec::Vec<ColorInfo>,
}
/// Stores image properties, such as dominant colors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageProperties {
    /// If present, dominant colors completed successfully.
    #[prost(message, optional, tag = "1")]
    pub dominant_colors: ::core::option::Option<DominantColorsAnnotation>,
}
/// Single crop hint that is used to generate a new crop when serving an image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHint {
    /// The bounding polygon for the crop region. The coordinates of the bounding
    /// box are in the original image's scale.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// Confidence of this being a salient region.  Range [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// Fraction of importance of this salient region with respect to the original
    /// image.
    #[prost(float, tag = "3")]
    pub importance_fraction: f32,
}
/// Set of crop hints that are used to generate new crops when serving images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsAnnotation {
    /// Crop hint results.
    #[prost(message, repeated, tag = "1")]
    pub crop_hints: ::prost::alloc::vec::Vec<CropHint>,
}
/// Parameters for crop hints annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsParams {
    /// Aspect ratios in floats, representing the ratio of the width to the height
    /// of the image. For example, if the desired aspect ratio is 4/3, the
    /// corresponding float value should be 1.33333.  If not specified, the
    /// best possible crop is returned. The number of provided aspect ratios is
    /// limited to a maximum of 16; any aspect ratios provided after the 16th are
    /// ignored.
    #[prost(float, repeated, tag = "1")]
    pub aspect_ratios: ::prost::alloc::vec::Vec<f32>,
}
/// Parameters for web detection request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetectionParams {
    /// Whether to include results derived from the geo information in the image.
    #[prost(bool, tag = "2")]
    pub include_geo_results: bool,
}
/// Parameters for text detections. This is used to control TEXT_DETECTION and
/// DOCUMENT_TEXT_DETECTION features.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDetectionParams {
    /// By default, Cloud Vision API only includes confidence score for
    /// DOCUMENT_TEXT_DETECTION result. Set the flag to true to include confidence
    /// score for TEXT_DETECTION as well.
    #[prost(bool, tag = "9")]
    pub enable_text_detection_confidence_score: bool,
    /// A list of advanced OCR options to fine-tune OCR behavior.
    #[prost(string, repeated, tag = "11")]
    pub advanced_ocr_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Image context and/or feature-specific parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageContext {
    /// Not used.
    #[prost(message, optional, tag = "1")]
    pub lat_long_rect: ::core::option::Option<LatLongRect>,
    /// List of languages to use for TEXT_DETECTION. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Text detection returns an
    /// error if one or more of the specified languages is not one of the
    /// [supported languages](<https://cloud.google.com/vision/docs/languages>).
    #[prost(string, repeated, tag = "2")]
    pub language_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Parameters for crop hints annotation request.
    #[prost(message, optional, tag = "4")]
    pub crop_hints_params: ::core::option::Option<CropHintsParams>,
    /// Parameters for face recognition.
    #[prost(message, optional, tag = "10")]
    pub face_recognition_params: ::core::option::Option<FaceRecognitionParams>,
    /// Parameters for product search.
    #[prost(message, optional, tag = "5")]
    pub product_search_params: ::core::option::Option<ProductSearchParams>,
    /// Parameters for web detection.
    #[prost(message, optional, tag = "6")]
    pub web_detection_params: ::core::option::Option<WebDetectionParams>,
    /// Parameters for text detection and document text detection.
    #[prost(message, optional, tag = "12")]
    pub text_detection_params: ::core::option::Option<TextDetectionParams>,
}
/// Request for performing Google Cloud Vision API tasks over a user-provided
/// image, with user-requested features, and with context information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageRequest {
    /// The image to be processed.
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
    /// Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<Feature>,
    /// Additional context that may accompany the image.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::core::option::Option<ImageContext>,
}
/// If an image was produced from a file (e.g. a PDF), this message gives
/// information about the source of that image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAnnotationContext {
    /// The URI of the file used to produce the image.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// If the file was a PDF or TIFF, this field gives the page number within
    /// the file used to produce the image.
    #[prost(int32, tag = "2")]
    pub page_number: i32,
}
/// Response to an image annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageResponse {
    /// If present, face detection has completed successfully.
    #[prost(message, repeated, tag = "1")]
    pub face_annotations: ::prost::alloc::vec::Vec<FaceAnnotation>,
    /// If present, landmark detection has completed successfully.
    #[prost(message, repeated, tag = "2")]
    pub landmark_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, logo detection has completed successfully.
    #[prost(message, repeated, tag = "3")]
    pub logo_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, label detection has completed successfully.
    #[prost(message, repeated, tag = "4")]
    pub label_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, localized object detection has completed successfully.
    /// This will be sorted descending by confidence score.
    #[prost(message, repeated, tag = "22")]
    pub localized_object_annotations: ::prost::alloc::vec::Vec<
        LocalizedObjectAnnotation,
    >,
    /// If present, text (OCR) detection has completed successfully.
    #[prost(message, repeated, tag = "5")]
    pub text_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, text (OCR) detection or document (OCR) text detection has
    /// completed successfully.
    /// This annotation provides the structural hierarchy for the OCR detected
    /// text.
    #[prost(message, optional, tag = "12")]
    pub full_text_annotation: ::core::option::Option<TextAnnotation>,
    /// If present, safe-search annotation has completed successfully.
    #[prost(message, optional, tag = "6")]
    pub safe_search_annotation: ::core::option::Option<SafeSearchAnnotation>,
    /// If present, image properties were extracted successfully.
    #[prost(message, optional, tag = "8")]
    pub image_properties_annotation: ::core::option::Option<ImageProperties>,
    /// If present, crop hints have completed successfully.
    #[prost(message, optional, tag = "11")]
    pub crop_hints_annotation: ::core::option::Option<CropHintsAnnotation>,
    /// If present, web detection has completed successfully.
    #[prost(message, optional, tag = "13")]
    pub web_detection: ::core::option::Option<WebDetection>,
    /// If present, product search has completed successfully.
    #[prost(message, optional, tag = "14")]
    pub product_search_results: ::core::option::Option<ProductSearchResults>,
    /// If set, represents the error message for the operation.
    /// Note that filled-in image annotations are guaranteed to be
    /// correct, even when `error` is set.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// If present, contextual information is needed to understand where this image
    /// comes from.
    #[prost(message, optional, tag = "21")]
    pub context: ::core::option::Option<ImageAnnotationContext>,
}
/// Multiple image annotation requests are batched into a single service call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesRequest {
    /// Required. Individual image annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<AnnotateImageRequest>,
}
/// Response to a batch image annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesResponse {
    /// Individual responses to image annotation requests within the batch.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<AnnotateImageResponse>,
}
/// A request to annotate one single file, e.g. a PDF, TIFF or GIF file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateFileRequest {
    /// Required. Information about the input file.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Required. Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<Feature>,
    /// Additional context that may accompany the image(s) in the file.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::core::option::Option<ImageContext>,
    /// Pages of the file to perform image annotation.
    ///
    /// Pages starts from 1, we assume the first page of the file is page 1.
    /// At most 5 pages are supported per request. Pages can be negative.
    ///
    /// Page 1 means the first page.
    /// Page 2 means the second page.
    /// Page -1 means the last page.
    /// Page -2 means the second to the last page.
    ///
    /// If the file is GIF instead of PDF or TIFF, page refers to GIF frames.
    ///
    /// If this field is empty, by default the service performs image annotation
    /// for the first 5 pages of the file.
    #[prost(int32, repeated, tag = "4")]
    pub pages: ::prost::alloc::vec::Vec<i32>,
}
/// Response to a single file annotation request. A file may contain one or more
/// images, which individually have their own responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateFileResponse {
    /// Information about the file for which this response is generated.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Individual responses to images found within the file. This field will be
    /// empty if the `error` field is set.
    #[prost(message, repeated, tag = "2")]
    pub responses: ::prost::alloc::vec::Vec<AnnotateImageResponse>,
    /// This field gives the total number of pages in the file.
    #[prost(int32, tag = "3")]
    pub total_pages: i32,
    /// If set, represents the error message for the failed request. The
    /// `responses` field will not be set in this case.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A list of requests to annotate files using the BatchAnnotateFiles API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateFilesRequest {
    /// Required. The list of file annotation requests. Right now we support only
    /// one AnnotateFileRequest in BatchAnnotateFilesRequest.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<AnnotateFileRequest>,
}
/// A list of file annotation responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateFilesResponse {
    /// The list of file annotation responses, each response corresponding to each
    /// AnnotateFileRequest in BatchAnnotateFilesRequest.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<AnnotateFileResponse>,
}
/// An offline file annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncAnnotateFileRequest {
    /// Required. Information about the input file.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Required. Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<Feature>,
    /// Additional context that may accompany the image(s) in the file.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::core::option::Option<ImageContext>,
    /// Required. The desired output location and metadata (e.g. format).
    #[prost(message, optional, tag = "4")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// The response for a single offline file annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncAnnotateFileResponse {
    /// The output location and metadata from AsyncAnnotateFileRequest.
    #[prost(message, optional, tag = "1")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Request for async image annotation for a list of images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateImagesRequest {
    /// Required. Individual image annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<AnnotateImageRequest>,
    /// Required. The desired output location and metadata (e.g. format).
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Response to an async batch image annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateImagesResponse {
    /// The output location and metadata from AsyncBatchAnnotateImagesRequest.
    #[prost(message, optional, tag = "1")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Multiple async file annotation requests are batched into a single service
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateFilesRequest {
    /// Required. Individual async file annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<AsyncAnnotateFileRequest>,
}
/// Response to an async batch file annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateFilesResponse {
    /// The list of file annotation responses, one for each request in
    /// AsyncBatchAnnotateFilesRequest.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<AsyncAnnotateFileResponse>,
}
/// The desired input location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// The Google Cloud Storage location to read the input from.
    #[prost(message, optional, tag = "1")]
    pub gcs_source: ::core::option::Option<GcsSource>,
    /// File content, represented as a stream of bytes.
    /// Note: As with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    ///
    /// Currently, this field only works for BatchAnnotateFiles requests. It does
    /// not work for AsyncBatchAnnotateFiles requests.
    #[prost(bytes = "vec", tag = "3")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// The type of the file. Currently only "application/pdf", "image/tiff" and
    /// "image/gif" are supported. Wildcards are not supported.
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// The desired output location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The Google Cloud Storage location to write the output(s) to.
    #[prost(message, optional, tag = "1")]
    pub gcs_destination: ::core::option::Option<GcsDestination>,
    /// The max number of response protos to put into each output JSON file on
    /// Google Cloud Storage.
    /// The valid range is [1, 100]. If not specified, the default value is 20.
    ///
    /// For example, for one pdf file with 100 pages, 100 response protos will
    /// be generated. If `batch_size` = 20, then 5 json files each
    /// containing 20 response protos will be written under the prefix
    /// `gcs_destination`.`uri`.
    ///
    /// Currently, batch_size only applies to GcsDestination, with potential future
    /// support for other output configurations.
    #[prost(int32, tag = "2")]
    pub batch_size: i32,
}
/// The Google Cloud Storage location where the input will be read from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Google Cloud Storage URI for the input file. This must only be a
    /// Google Cloud Storage object. Wildcards are not currently supported.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location where the output will be written to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Google Cloud Storage URI prefix where the results will be stored. Results
    /// will be in JSON format and preceded by its corresponding input URI prefix.
    /// This field can either represent a gcs file prefix or gcs directory. In
    /// either case, the uri should be unique because in order to get all of the
    /// output files, you will need to do a wildcard gcs search on the uri prefix
    /// you provide.
    ///
    /// Examples:
    ///
    /// *    File Prefix: gs://bucket-name/here/filenameprefix   The output files
    /// will be created in gs://bucket-name/here/ and the names of the
    /// output files will begin with "filenameprefix".
    ///
    /// *    Directory Prefix: gs://bucket-name/some/location/   The output files
    /// will be created in gs://bucket-name/some/location/ and the names of the
    /// output files could be anything because there was no filename prefix
    /// specified.
    ///
    /// If multiple outputs, each response is still AnnotateFileResponse, each of
    /// which contains some subset of the full list of AnnotateImageResponse.
    /// Multiple outputs can happen if, for example, the output JSON is too large
    /// and overflows into multiple sharded files.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Contains metadata for the BatchAnnotateImages operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Current state of the batch operation.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The time when the batch request was received.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the operation result was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Batch operation states.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Invalid.
        Unspecified = 0,
        /// Request is received.
        Created = 1,
        /// Request is actively being processed.
        Running = 2,
        /// The batch processing is done.
        Done = 3,
        /// The batch processing was cancelled.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Created => "CREATED",
                State::Running => "RUNNING",
                State::Done => "DONE",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATED" => Some(Self::Created),
                "RUNNING" => Some(Self::Running),
                "DONE" => Some(Self::Done),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// A bucketized representation of likelihood, which is intended to give clients
/// highly stable results across model upgrades.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unknown likelihood.
    Unknown = 0,
    /// It is very unlikely.
    VeryUnlikely = 1,
    /// It is unlikely.
    Unlikely = 2,
    /// It is possible.
    Possible = 3,
    /// It is likely.
    Likely = 4,
    /// It is very likely.
    VeryLikely = 5,
}
impl Likelihood {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Likelihood::Unknown => "UNKNOWN",
            Likelihood::VeryUnlikely => "VERY_UNLIKELY",
            Likelihood::Unlikely => "UNLIKELY",
            Likelihood::Possible => "POSSIBLE",
            Likelihood::Likely => "LIKELY",
            Likelihood::VeryLikely => "VERY_LIKELY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "VERY_UNLIKELY" => Some(Self::VeryUnlikely),
            "UNLIKELY" => Some(Self::Unlikely),
            "POSSIBLE" => Some(Self::Possible),
            "LIKELY" => Some(Self::Likely),
            "VERY_LIKELY" => Some(Self::VeryLikely),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod image_annotator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that performs Google Cloud Vision API detection tasks over client
    /// images, such as face, landmark, logo, label, and text detection. The
    /// ImageAnnotator service returns detected entities from the images.
    #[derive(Debug, Clone)]
    pub struct ImageAnnotatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ImageAnnotatorClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ImageAnnotatorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ImageAnnotatorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ImageAnnotatorClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Run image detection and annotation for a batch of images.
        pub async fn batch_annotate_images(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchAnnotateImagesRequest>,
        ) -> Result<tonic::Response<super::BatchAnnotateImagesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ImageAnnotator/BatchAnnotateImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Service that performs image detection and annotation for a batch of files.
        /// Now only "application/pdf", "image/tiff" and "image/gif" are supported.
        ///
        /// This service will extract at most 5 (customers can specify which 5 in
        /// AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each
        /// file provided and perform detection and annotation for each image
        /// extracted.
        pub async fn batch_annotate_files(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchAnnotateFilesRequest>,
        ) -> Result<tonic::Response<super::BatchAnnotateFilesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ImageAnnotator/BatchAnnotateFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Run asynchronous image detection and annotation for a list of images.
        ///
        /// Progress and results can be retrieved through the
        /// `google.longrunning.Operations` interface.
        /// `Operation.metadata` contains `OperationMetadata` (metadata).
        /// `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).
        ///
        /// This service will write image annotation outputs to json files in customer
        /// GCS bucket, each json file containing BatchAnnotateImagesResponse proto.
        pub async fn async_batch_annotate_images(
            &mut self,
            request: impl tonic::IntoRequest<super::AsyncBatchAnnotateImagesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ImageAnnotator/AsyncBatchAnnotateImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Run asynchronous image detection and annotation for a list of generic
        /// files, such as PDF files, which may contain multiple pages and multiple
        /// images per page. Progress and results can be retrieved through the
        /// `google.longrunning.Operations` interface.
        /// `Operation.metadata` contains `OperationMetadata` (metadata).
        /// `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).
        pub async fn async_batch_annotate_files(
            &mut self,
            request: impl tonic::IntoRequest<super::AsyncBatchAnnotateFilesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p4beta1.ImageAnnotator/AsyncBatchAnnotateFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
