# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_asset**](DefaultApi.md#create_asset) | **post** /assets | Create a new asset
[**create_asset_type**](DefaultApi.md#create_asset_type) | **post** /types | Create a new asset type
[**create_lease**](DefaultApi.md#create_lease) | **put** /assets/{asset_id}/lease | Create a new lease for this asset
[**create_tag**](DefaultApi.md#create_tag) | **post** /assets/{asset_id}/tags | Create a new tag
[**create_tag_type**](DefaultApi.md#create_tag_type) | **post** /types/{asset_type_id}/tag-types | Create a new tag type
[**delete_asset**](DefaultApi.md#delete_asset) | **delete** /assets/{asset_id} | Delete an asset and all tags associated with it
[**delete_asset_type**](DefaultApi.md#delete_asset_type) | **delete** /types/{asset_type_id} | Delete an asset type and all assets and tags associated with it
[**delete_lease**](DefaultApi.md#delete_lease) | **delete** /assets/{asset_id}/lease | Release a lease ahead of its end time
[**list_asset_types**](DefaultApi.md#list_asset_types) | **get** /types | List all asset types
[**list_assets**](DefaultApi.md#list_assets) | **get** /assets | List all assets
[**list_sub_assets**](DefaultApi.md#list_sub_assets) | **get** /types/{asset_type_id}/assets | List assets that belong to an asset type
[**list_tag_types**](DefaultApi.md#list_tag_types) | **get** /types/{asset_type_id}/tag-types | List tag types that belong to an asset type
[**list_tags**](DefaultApi.md#list_tags) | **get** /assets/{asset_id}/tags | List tags that belong to an asset
[**show_asset**](DefaultApi.md#show_asset) | **get** /assets/{asset_id} | Show details of an asset
[**show_asset_type**](DefaultApi.md#show_asset_type) | **get** /types/{asset_type_id} | Show details of an asset type
[**show_lease**](DefaultApi.md#show_lease) | **get** /assets/{asset_id}/lease | Show details of an asset's lease
[**show_tag**](DefaultApi.md#show_tag) | **get** /assets/{asset_id}/tags/{tag_type_id} | Show details of a tag
[**show_tag_type**](DefaultApi.md#show_tag_type) | **get** /types/{asset_type_id}/tag-types/{tag_type_id} | Show details of a tag type


# **create_asset**
> ::models::Asset create_asset(ctx, create_asset)
Create a new asset

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_asset** | [**CreateAsset**](CreateAsset.md)| Asset to create | 

### Return type

[**::models::Asset**](Asset.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_asset_type**
> ::models::AssetType create_asset_type(ctx, create_asset_type)
Create a new asset type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_asset_type** | [**CreateAssetType**](CreateAssetType.md)| Asset type to create | 

### Return type

[**::models::AssetType**](AssetType.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_lease**
> ::models::Lease create_lease(ctx, asset_id, create_lease)
Create a new lease for this asset

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 
  **create_lease** | [**CreateLease**](CreateLease.md)| Lease to create | 

### Return type

[**::models::Lease**](Lease.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_tag**
> ::models::Tag create_tag(ctx, asset_id, create_tag)
Create a new tag

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 
  **create_tag** | [**CreateTag**](CreateTag.md)| Tag to create | 

### Return type

[**::models::Tag**](Tag.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_tag_type**
> ::models::TagType create_tag_type(ctx, asset_type_id, create_tag_type)
Create a new tag type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 
  **create_tag_type** | [**CreateTagType**](CreateTagType.md)| Tag type to create | 

### Return type

[**::models::TagType**](TagType.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_asset**
> delete_asset(ctx, asset_id)
Delete an asset and all tags associated with it

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 

### Return type

 (empty response body)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_asset_type**
> delete_asset_type(ctx, asset_type_id)
Delete an asset type and all assets and tags associated with it

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 

### Return type

 (empty response body)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_lease**
> delete_lease(ctx, asset_id)
Release a lease ahead of its end time

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 

### Return type

 (empty response body)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_asset_types**
> ::models::AssetTypes list_asset_types(ctx, )
List all asset types

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AssetTypes**](AssetTypes.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_assets**
> ::models::Assets list_assets(ctx, )
List all assets

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Assets**](Assets.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sub_assets**
> ::models::Assets list_sub_assets(ctx, asset_type_id)
List assets that belong to an asset type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 

### Return type

[**::models::Assets**](Assets.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_tag_types**
> ::models::TagTypes list_tag_types(ctx, asset_type_id)
List tag types that belong to an asset type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 

### Return type

[**::models::TagTypes**](TagTypes.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_tags**
> ::models::Tags list_tags(ctx, asset_id)
List tags that belong to an asset

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 

### Return type

[**::models::Tags**](Tags.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show_asset**
> ::models::Asset show_asset(ctx, asset_id)
Show details of an asset

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 

### Return type

[**::models::Asset**](Asset.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show_asset_type**
> ::models::AssetType show_asset_type(ctx, asset_type_id)
Show details of an asset type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 

### Return type

[**::models::AssetType**](AssetType.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show_lease**
> ::models::Lease show_lease(ctx, asset_id)
Show details of an asset's lease

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 

### Return type

[**::models::Lease**](Lease.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show_tag**
> ::models::Tag show_tag(ctx, asset_id, tag_type_id)
Show details of a tag

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_id** | **i32**| Identifier of the asset | 
  **tag_type_id** | **i32**| Identifier of the tag type | 

### Return type

[**::models::Tag**](Tag.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show_tag_type**
> ::models::TagType show_tag_type(ctx, asset_type_id, tag_type_id)
Show details of a tag type

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **asset_type_id** | **i32**| Identifier of the asset type | 
  **tag_type_id** | **i32**| Identifier of the tag type | 

### Return type

[**::models::TagType**](TagType.md)

### Authorization

[XBellhopEmail](../README.md#XBellhopEmail)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

