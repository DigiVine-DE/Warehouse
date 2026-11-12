# terrain_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneTerrain**](terrain_api.md#deleteOneTerrain) | **DELETE** /terrain/{TerrainId} | Removes a specific stored Terrain Model from the collection
**deleteTerrain**](terrain_api.md#deleteTerrain) | **DELETE** /terrain | Can't delete complete collection
**getAllTerrains**](terrain_api.md#getAllTerrains) | **GET** /terrain | Returns a list of (all) stored Terrain Models
**getOneTerrain**](terrain_api.md#getOneTerrain) | **GET** /terrain/{TerrainId} | Returns a specific stored Terrain Model from the collection


# **deleteOneTerrain**
> deleteOneTerrain(ctx, ctx, terrain_id)
Removes a specific stored Terrain Model from the collection

Removes a specific stored Terrain Model from the collection. The parameter `TerrainId` is required, to delete the specific stored Terrain Model. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **terrain_id** | **i64**| The numeric ID of the terrain model | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTerrain**
> deleteTerrain(ctx, ctx, )
Can't delete complete collection

You can't delete the complete collection of Terrain Model data. You can delete one specific Terrain Model with `DELETE /terrain/{TerrainId}` (`deleteOneTerrain`). 

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllTerrains**
> Vec<models::Terrain> getAllTerrains(ctx, ctx, optional)
Returns a list of (all) stored Terrain Models

Returns a list of (all) stored Terrain Models. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Terrain Models will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Terrain>**](Terrain.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneTerrain**
> models::Terrain getOneTerrain(ctx, ctx, terrain_id)
Returns a specific stored Terrain Model from the collection

Returns a specific stored Terrain Model from the collection. The parameter `TerrainId` is required, to get the specific stored Terrain Model. If the operation was successful (status code `200`), the specific Terrain Model will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **terrain_id** | **i64**| The numeric ID of the terrain model | 

### Return type

[**models::Terrain**](Terrain.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

