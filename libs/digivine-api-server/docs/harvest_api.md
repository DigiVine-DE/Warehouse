# harvest_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteCompletedHarvestTask**](harvest_api.md#deleteCompletedHarvestTask) | **DELETE** /harvest/complete | Can't delete complete collection
**deleteOneCompletedHarvestTask**](harvest_api.md#deleteOneCompletedHarvestTask) | **DELETE** /harvest/complete/{HarvestTaskId} | Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
**deleteOneScheduledHarvestTask**](harvest_api.md#deleteOneScheduledHarvestTask) | **DELETE** /harvest/task/{HarvestTaskId} | Removes a specific stored Harvest Task from the scheduled collection
**deleteScheduledHarvestTask**](harvest_api.md#deleteScheduledHarvestTask) | **DELETE** /harvest/task | Can't delete complete collection
**getAllCompletedHarvestTasks**](harvest_api.md#getAllCompletedHarvestTasks) | **GET** /harvest/complete | Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements
**getAllScheduledHarvestTasks**](harvest_api.md#getAllScheduledHarvestTasks) | **GET** /harvest/task | Returns a list of (all) stored scheduled Harvest Tasks
**getOneCompletedHarvestTask**](harvest_api.md#getOneCompletedHarvestTask) | **GET** /harvest/complete/{HarvestTaskId} | Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
**getOneScheduledHarvestTask**](harvest_api.md#getOneScheduledHarvestTask) | **GET** /harvest/task/{HarvestTaskId} | Returns a specific stored Harvest Task from the scheduled collection
**postCompletedHarvestTask**](harvest_api.md#postCompletedHarvestTask) | **POST** /harvest/complete | Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection
**postScheduledHarvestTask**](harvest_api.md#postScheduledHarvestTask) | **POST** /harvest/task | Adds a new Harvest Task to the scheduled collection


# **deleteCompletedHarvestTask**
> deleteCompletedHarvestTask(ctx, ctx, )
Can't delete complete collection

You can't delete the complete completed collection of Harvest Tasks and Harvests with Harvest Measurements. You can delete one specific completed Harvest Task with `DELETE /harvest/complete/{HarvestTaskId}` (`deleteOneCompletedHarvestTask`). Or you can delete one specific scheduled Harvest Task with `DELETE /harvest/task/{HarvestTaskId}` (`deleteOneScheduledHarvestTask`). 

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

# **deleteOneCompletedHarvestTask**
> deleteOneCompletedHarvestTask(ctx, ctx, harvest_task_id)
Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection

Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection. The parameter `HarvestTaskId` is required, to delete the specific stored Harvest. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_task_id** | **i64**| The numeric ID of the Harvest Task | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOneScheduledHarvestTask**
> deleteOneScheduledHarvestTask(ctx, ctx, harvest_task_id)
Removes a specific stored Harvest Task from the scheduled collection

Removes a specific stored Harvest Task from the scheduled collection. The parameter `HarvestTaskId` is required, to delete the specific stored Harvest Task. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_task_id** | **i64**| The numeric ID of the Harvest Task | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteScheduledHarvestTask**
> deleteScheduledHarvestTask(ctx, ctx, )
Can't delete complete collection

You can't delete the complete scheduled collection of Harvest Tasks. You can delete one specific scheduled Harvest Task with `DELETE /harvest/task/{HarvestTaskId}` (`deleteOneScheduledHarvestTask`). Or you can delete one specific completed Harvest Task with `DELETE /harvest/complete/{HarvestTaskId}` (`deleteOneCompletedHarvestTask`). 

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

# **getAllCompletedHarvestTasks**
> Vec<models::HarvestOrTask> getAllCompletedHarvestTasks(ctx, ctx, optional)
Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements

Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements. With the optional parameter `Box2d`, only the completed Harvest Tasks that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Harvests will be returned. 

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
 **box2d** | [**f64**](f64.md)| The two-dimensional geometry bounding box | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::HarvestOrTask>**](HarvestOrTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllScheduledHarvestTasks**
> Vec<models::HarvestTask> getAllScheduledHarvestTasks(ctx, ctx, optional)
Returns a list of (all) stored scheduled Harvest Tasks

Returns a list of (all) stored scheduled Harvest Tasks. With the optional parameter `Box2d`, only the Harvest Tasks that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Harvest Tasks will be returned. 

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
 **box2d** | [**f64**](f64.md)| The two-dimensional geometry bounding box | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::HarvestTask>**](HarvestTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneCompletedHarvestTask**
> models::HarvestOrTask getOneCompletedHarvestTask(ctx, ctx, harvest_task_id)
Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection

Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection. The parameter `HarvestTaskId` is required, to get the specific stored Harvest. If the operation was successful (status code `200`), the specific Harvest will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_task_id** | **i64**| The numeric ID of the Harvest Task | 

### Return type

[**models::HarvestOrTask**](HarvestOrTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneScheduledHarvestTask**
> models::HarvestTask getOneScheduledHarvestTask(ctx, ctx, harvest_task_id)
Returns a specific stored Harvest Task from the scheduled collection

Returns a specific stored Harvest Task from the scheduled collection. The parameter `HarvestTaskId` is required, to get the specific stored Harvest Task. If the operation was successful (status code `200`), the specific Harvest Task will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_task_id** | **i64**| The numeric ID of the Harvest Task | 

### Return type

[**models::HarvestTask**](HarvestTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postCompletedHarvestTask**
> models::InlineResponse2015 postCompletedHarvestTask(ctx, ctx, harvest_or_task)
Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection

Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection. In the request body you have to add either the new Harvest Task or the new Harvest with Harvest Measurements, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Harvest will be returned. Note: All Harvests with the same ID - as the ID specified here in the request body - are removed from the list of scheduled Harvest Tasks! 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_or_task** | [**HarvestOrTask**](HarvestOrTask.md)| Either a Harvest Task or a Harvest with Harvest Measurements to be added to the completed collection | 

### Return type

[**models::InlineResponse2015**](inline_response_201_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postScheduledHarvestTask**
> models::InlineResponse2015 postScheduledHarvestTask(ctx, ctx, harvest_task)
Adds a new Harvest Task to the scheduled collection

Adds a new Harvest Task to the scheduled collection. In the request body you have to add the new Harvest Task, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Harvest Task will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **harvest_task** | [**HarvestTask**](HarvestTask.md)| A Harvest Task to be added to the scheduled collection | 

### Return type

[**models::InlineResponse2015**](inline_response_201_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

