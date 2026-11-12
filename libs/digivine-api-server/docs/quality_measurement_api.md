# quality_measurement_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneQualityMeasurement**](quality_measurement_api.md#deleteOneQualityMeasurement) | **DELETE** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId} | Removes a specific stored Quality Measurement from the collection
**deleteQualityMeasurement**](quality_measurement_api.md#deleteQualityMeasurement) | **DELETE** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement | Can't delete complete collection
**getAllQualityMeasurements**](quality_measurement_api.md#getAllQualityMeasurements) | **GET** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement | Returns a list of (all) stored Quality Measurements
**getOneQualityMeasurement**](quality_measurement_api.md#getOneQualityMeasurement) | **GET** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId} | Returns a specific stored Quality Measurement from the collection
**postQualityMeasurement**](quality_measurement_api.md#postQualityMeasurement) | **POST** /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement | Adds a new Quality Measurement to the collection


# **deleteOneQualityMeasurement**
> deleteOneQualityMeasurement(ctx, ctx, field_id, planter_run_id, plant_id, quality_measurement_id)
Removes a specific stored Quality Measurement from the collection

Removes a specific stored Quality Measurement from the collection. The parameters `FieldId`, `PlanterRunId`, `PlantId` and `QualityMeasurementId` are required, to delete the specific stored Quality Measurement. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 
  **quality_measurement_id** | **i64**| The numeric ID of the quality measurement | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteQualityMeasurement**
> deleteQualityMeasurement(ctx, ctx, field_id, planter_run_id, plant_id)
Can't delete complete collection

You can't delete the complete collection of Quality Measurement data. You can delete one specific Quality Measurement with `DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId}` (`deleteOneQualityMeasurement`). 

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

# **getAllQualityMeasurements**
> Vec<models::QualityMeasurement> getAllQualityMeasurements(ctx, ctx, field_id, planter_run_id, plant_id, optional)
Returns a list of (all) stored Quality Measurements

Returns a list of (all) stored Quality Measurements. The parameters `FieldId`, `PlanterRunId` and `PlantId` are required, to get the specific stored Quality Measurements. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Quality Measurements will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_id** | **i64**| The numeric ID of the field | 
 **planter_run_id** | **i64**| The numeric ID of the planter run | 
 **plant_id** | **i64**| The numeric ID of the plant | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::QualityMeasurement>**](QualityMeasurement.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneQualityMeasurement**
> models::QualityMeasurement getOneQualityMeasurement(ctx, ctx, field_id, planter_run_id, plant_id, quality_measurement_id)
Returns a specific stored Quality Measurement from the collection

Returns a specific stored Quality Measurement from the collection. The parameters `FieldId`, `PlanterRunId`, `PlantId` and `QualityMeasurementId` are required, to get the specific stored Quality Measurement. If the operation was successful (status code `200`), the specific Quality Measurement will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 
  **quality_measurement_id** | **i64**| The numeric ID of the quality measurement | 

### Return type

[**models::QualityMeasurement**](QualityMeasurement.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postQualityMeasurement**
> models::InlineResponse2014 postQualityMeasurement(ctx, ctx, field_id, planter_run_id, plant_id, quality_measurement)
Adds a new Quality Measurement to the collection

Adds a new Quality Measurement to the collection. In the request body you have to add the new Quality Measurement, which to be added. The parameters `FieldId`, `PlanterRunId` and `PlantId` are required, to add a Quality Measurement. If the operation was successful (status code `200`), the numeric ID of the new Quality Measurement will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **planter_run_id** | **i64**| The numeric ID of the planter run | 
  **plant_id** | **i64**| The numeric ID of the plant | 
  **quality_measurement** | [**QualityMeasurement**](QualityMeasurement.md)| A Quality Measurement to be added to the collection | 

### Return type

[**models::InlineResponse2014**](inline_response_201_4.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

