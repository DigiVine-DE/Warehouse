# plant_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOnePlant**](plant_api.md#deleteOnePlant) | **DELETE** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId} | Removes a specific stored Plant from the collection
**deletePlant**](plant_api.md#deletePlant) | **DELETE** /field/{FieldId}/planter/{PlanterRunId}/plant | Can't delete complete collection
**getAllPlants**](plant_api.md#getAllPlants) | **GET** /field/{FieldId}/planter/{PlanterRunId}/plant | Returns a list of (all) stored Plants
**getOnePlant**](plant_api.md#getOnePlant) | **GET** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId} | Returns a specific stored Plant from the collection
**patchOnePlant**](plant_api.md#patchOnePlant) | **PATCH** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId} | Patches a specific stored Plant
**postPlant**](plant_api.md#postPlant) | **POST** /field/{FieldId}/planter/{PlanterRunId}/plant | Adds a new Plant to the collection


# **deleteOnePlant**
> deleteOnePlant(ctx, ctx, field_id, planter_run_id, plant_id)
Removes a specific stored Plant from the collection

Removes a specific stored Plant from the collection. The parameters `FieldId`, `PlanterRunId` and `PlantId` are required, to delete the specific stored Plant. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deletePlant**
> deletePlant(ctx, ctx, field_id, planter_run_id)
Can't delete complete collection

You can't delete the complete collection of Plants data. The parameters `FieldId` and `PlanterRunId` are required. You can delete one specific Plant with `DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}` (`deleteOnePlant`). 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllPlants**
> Vec<models::Plant> getAllPlants(ctx, ctx, field_id, planter_run_id, optional)
Returns a list of (all) stored Plants

Returns a list of (all) stored Plants. The parameters `FieldId` and `PlanterRunId` are required, to get the specific stored Plants. With the optional parameter `Box2d`, only the Plants that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Plants will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_id** | **i64**| The numeric ID of the field | 
 **planter_run_id** | **i64**| The numeric ID of the planter run | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]
 **box2d** | [**f64**](f64.md)| The two-dimensional geometry bounding box | 

### Return type

[**Vec<models::Plant>**](Plant.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOnePlant**
> models::Plant getOnePlant(ctx, ctx, field_id, planter_run_id, plant_id)
Returns a specific stored Plant from the collection

Returns a specific stored Plant from the collection. The parameters `FieldId`, `PlanterRunId` and `PlantId` are required, to get the specific stored Plant. If the operation was successful (status code `200`), the specific Plant will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 

### Return type

[**models::Plant**](Plant.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchOnePlant**
> models::Plant patchOnePlant(ctx, ctx, field_id, planter_run_id, plant_id, plant_patch)
Patches a specific stored Plant

Patches a specific stored Plant. The parameters `FieldId`, `PlanterRunId` and `PlantId` are required, to replace the specific stored Plant by the new one. Also the new data (`PlantPatch`) is required. If the operation was successful (status code `200`), the new Plant will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 
  **plant_patch** | [**PlantPatch**](PlantPatch.md)| The updated properties to patch | 

### Return type

[**models::Plant**](Plant.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postPlant**
> models::InlineResponse2013 postPlant(ctx, ctx, field_id, planter_run_id, plant)
Adds a new Plant to the collection

Adds a new Plant to the collection. In the request body you have to add the new Plant, which to be added. The parameters `FieldId` and `PlanterRunId` are required, to add a Plant. If the operation was successful (status code `200`), the numeric ID of the new Plant will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant** | [**Plant**](Plant.md)| A plant to be added to the collection | 

### Return type

[**models::InlineResponse2013**](inline_response_201_3.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

