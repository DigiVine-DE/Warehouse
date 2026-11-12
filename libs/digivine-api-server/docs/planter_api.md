# planter_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOnePlanterRun**](planter_api.md#deleteOnePlanterRun) | **DELETE** /field/{FieldId}/planter/{PlanterRunId} | Removes a specific stored Planter Run from the collection
**deletePlanterRun**](planter_api.md#deletePlanterRun) | **DELETE** /field/{FieldId}/planter | Can't delete complete collection
**getAllPlanterRuns**](planter_api.md#getAllPlanterRuns) | **GET** /field/{FieldId}/planter | Returns a list of (all) stored Planter Runs
**getOnePlanterRun**](planter_api.md#getOnePlanterRun) | **GET** /field/{FieldId}/planter/{PlanterRunId} | Returns a specific stored Planter Run from the collection
**postPlanterRun**](planter_api.md#postPlanterRun) | **POST** /field/{FieldId}/planter | Adds a new Planter Run to the collection


# **deleteOnePlanterRun**
> deleteOnePlanterRun(ctx, ctx, field_id, planter_run_id)
Removes a specific stored Planter Run from the collection

Removes a specific stored Planter Run from the collection. The parameters `FieldId` and `PlanterRunId` are required, to delete the specific stored Planter Run. 

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

# **deletePlanterRun**
> deletePlanterRun(ctx, ctx, field_id)
Can't delete complete collection

You can't delete the complete collection of Planter Run data. You can delete one specific Planter Run with `DELETE /field/{FieldId}/planter/{PlanterRunId}` (`deleteOnePlanterRun`). 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllPlanterRuns**
> Vec<models::PlanterRun> getAllPlanterRuns(ctx, ctx, field_id, optional)
Returns a list of (all) stored Planter Runs

Returns a list of (all) stored Planter Runs. The parameter `FieldId` is required, to get the specific stored Planter Runs. With the optional parameter `Box2d`, only the Planter Runs that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Planter Runs will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_id** | **i64**| The numeric ID of the field | 
 **box2d** | [**f64**](f64.md)| The two-dimensional geometry bounding box | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::PlanterRun>**](PlanterRun.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOnePlanterRun**
> models::PlanterRun getOnePlanterRun(ctx, ctx, field_id, planter_run_id)
Returns a specific stored Planter Run from the collection

Returns a specific stored Planter Run from the collection. The parameters `FieldId` and `PlanterRunId` are required, to get the specific stored Planter Run. If the operation was successful (status code `200`), the specific Planter Run will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 

### Return type

[**models::PlanterRun**](PlanterRun.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postPlanterRun**
> models::InlineResponse2012 postPlanterRun(ctx, ctx, field_id, planter_run)
Adds a new Planter Run to the collection

Adds a new Planter Run to the collection. In the request body you have to add the new Planter Run, which to be added. The parameter `FieldId` is required, to add a Planter Run. If the operation was successful (status code `200`), the numeric ID of the new Planter Run will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run** | [**PlanterRun**](PlanterRun.md)| A Planter Run to be added to the collection | 

### Return type

[**models::InlineResponse2012**](inline_response_201_2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

