# spray_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteCompletedSprayTask**](spray_api.md#deleteCompletedSprayTask) | **DELETE** /spray/complete | Can't delete complete collection
**deleteOneCompletedSprayTask**](spray_api.md#deleteOneCompletedSprayTask) | **DELETE** /spray/complete/{SprayTaskId} | Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
**deleteOneScheduledSprayTask**](spray_api.md#deleteOneScheduledSprayTask) | **DELETE** /spray/task/{SprayTaskId} | Removes a specific stored Spray Task from the scheduled collection
**deleteScheduledSprayTask**](spray_api.md#deleteScheduledSprayTask) | **DELETE** /spray/task | Can't delete complete collection
**getAllCompletedSprayTasks**](spray_api.md#getAllCompletedSprayTasks) | **GET** /spray/complete | Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements
**getAllScheduledSprayTasks**](spray_api.md#getAllScheduledSprayTasks) | **GET** /spray/task | Returns a list of (all) stored scheduled Spray Tasks
**getOneCompletedSprayTask**](spray_api.md#getOneCompletedSprayTask) | **GET** /spray/complete/{SprayTaskId} | Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
**getOneScheduledSprayTask**](spray_api.md#getOneScheduledSprayTask) | **GET** /spray/task/{SprayTaskId} | Returns a specific stored Spray Task from the scheduled collection
**postCompletedSprayTask**](spray_api.md#postCompletedSprayTask) | **POST** /spray/complete | Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection
**postScheduledSprayTask**](spray_api.md#postScheduledSprayTask) | **POST** /spray/task | Adds a new Spray Task to the scheduled collection


# **deleteCompletedSprayTask**
> deleteCompletedSprayTask(ctx, ctx, )
Can't delete complete collection

You can't delete the complete completed collection of Spray Tasks and Sprays with Spray Measurements. You can delete one specific completed Spray Task with `DELETE /spray/complete/{SprayTaskId}` (`deleteOneCompletedSprayTask`). Or you can delete one specific scheduled Spray Task with `DELETE /spray/task/{SprayTaskId}` (`deleteOneScheduledSprayTask`). 

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

# **deleteOneCompletedSprayTask**
> deleteOneCompletedSprayTask(ctx, ctx, spray_task_id)
Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection

Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection. The parameter `SprayTaskId` is required, to delete the specific stored Spray. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task_id** | **i64**| The numeric ID of the Spray Task | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOneScheduledSprayTask**
> deleteOneScheduledSprayTask(ctx, ctx, spray_task_id)
Removes a specific stored Spray Task from the scheduled collection

Removes a specific stored Spray Task from the scheduled collection. The parameter `SprayTaskId` is required, to delete the specific stored Spray Task. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task_id** | **i64**| The numeric ID of the Spray Task | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteScheduledSprayTask**
> deleteScheduledSprayTask(ctx, ctx, )
Can't delete complete collection

You can't delete the complete scheduled collection of Spray Tasks. You can delete one specific scheduled Spray Task with `DELETE /spray/task/{SprayTaskId}` (`deleteOneScheduledSprayTask`). Or you can delete one specific completed Spray Task with `DELETE /spray/complete/{SprayTaskId}` (`deleteOneCompletedSprayTask`). 

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

# **getAllCompletedSprayTasks**
> Vec<models::SprayTask> getAllCompletedSprayTasks(ctx, ctx, optional)
Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements

Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements. With the optional parameter `Box2d`, only the completed Spray Tasks that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Sprays will be returned. 

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

[**Vec<models::SprayTask>**](SprayTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllScheduledSprayTasks**
> Vec<models::SprayTask> getAllScheduledSprayTasks(ctx, ctx, optional)
Returns a list of (all) stored scheduled Spray Tasks

Returns a list of (all) stored scheduled Spray Tasks. With the optional parameter `Box2d`, only the Spray Tasks that intersect the given bounding box will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Spray Tasks will be returned. 

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

[**Vec<models::SprayTask>**](SprayTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneCompletedSprayTask**
> models::SprayTask getOneCompletedSprayTask(ctx, ctx, spray_task_id)
Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection

Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection. The parameter `SprayTaskId` is required, to get the specific stored Spray. If the operation was successful (status code `200`), the specific Spray will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task_id** | **i64**| The numeric ID of the Spray Task | 

### Return type

[**models::SprayTask**](SprayTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneScheduledSprayTask**
> models::SprayTask getOneScheduledSprayTask(ctx, ctx, spray_task_id)
Returns a specific stored Spray Task from the scheduled collection

Returns a specific stored Spray Task from the scheduled collection. The parameter `SprayTaskId` is required, to get the specific stored Spray Task. If the operation was successful (status code `200`), the specific Spray Task will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task_id** | **i64**| The numeric ID of the Spray Task | 

### Return type

[**models::SprayTask**](SprayTask.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postCompletedSprayTask**
> models::InlineResponse2016 postCompletedSprayTask(ctx, ctx, spray_task)
Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection

Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection. In the request body you have to add either the new Spray Task or the new Spray with Spray Measurements, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Spray will be returned. Note: All Sprays with the same ID - as the ID specified here in the request body - are removed from the list of scheduled Spray Tasks! 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task** | [**SprayTask**](SprayTask.md)| Either a Spray Task or a Spray with Spray Measurements to be added to the completed collection | 

### Return type

[**models::InlineResponse2016**](inline_response_201_6.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postScheduledSprayTask**
> models::InlineResponse2016 postScheduledSprayTask(ctx, ctx, spray_task)
Adds a new Spray Task to the scheduled collection

Adds a new Spray Task to the scheduled collection. In the request body you have to add the new Spray Task, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Spray Task will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **spray_task** | [**SprayTask**](SprayTask.md)| A Spray Task to be added to the scheduled collection | 

### Return type

[**models::InlineResponse2016**](inline_response_201_6.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

