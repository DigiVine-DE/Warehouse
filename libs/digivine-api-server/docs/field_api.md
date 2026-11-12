# field_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteField**](field_api.md#deleteField) | **DELETE** /field | Can't delete complete collection
**deleteOneField**](field_api.md#deleteOneField) | **DELETE** /field/{FieldId} | Removes a specific stored Field from the collection
**getAllFields**](field_api.md#getAllFields) | **GET** /field | Returns a list of (all) stored Fields
**getOneField**](field_api.md#getOneField) | **GET** /field/{FieldId} | Returns a specific stored Field from the collection
**patchOneField**](field_api.md#patchOneField) | **PATCH** /field/{FieldId} | Patches a specific stored Field
**postField**](field_api.md#postField) | **POST** /field | Adds a new Field to the collection


# **deleteField**
> deleteField(ctx, ctx, )
Can't delete complete collection

You can't delete the complete collection of Field data. You can delete one specific Field with `DELETE /field/{FieldId}` (`deleteOneField`). 

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

# **deleteOneField**
> deleteOneField(ctx, ctx, field_id)
Removes a specific stored Field from the collection

Removes a specific stored Field from the collection. The parameter `FieldId` is required, to delete the specific stored Field. 

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

# **getAllFields**
> Vec<models::Field> getAllFields(ctx, ctx, optional)
Returns a list of (all) stored Fields

Returns a list of (all) stored Fields. With the optional parameter `Box2d` parameter, only the fields that intersect the given bounding box will be returned. With the optional parameter `FieldNameContains`, only the fields with names containing all specified keywords (case insensitive) will be returned. With the optional parameter `FruitNameContains`, only the fields with fruit names containing all specified keywords (case insensitive) will be returned. With the optional parameter `FuzzyFieldKeywords`, only the fields with at least one string attribute containing one of the specified keywords (case insensitive) will be returned. With the parameters `Limit` and `Offset` it can be specified, which data to return. The returned fields are ordered based on the number of string segments matched by the fuzzy keywords. Fields with case-sensitive matches receive a higher score. If two fields have identical scores, they are sorted by their field ID. If the operation was successful (status code `200`), an array with Fields will be returned. 

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
 **fuzzy_field_keywords** | [**String**](String.md)| A set of keywords used to filter the results of fields. Only fields with at least one string attribute containing one of the keywords (case insensitive) will be returned. | 
 **field_name_contains** | [**String**](String.md)| A set of keywords used to filter the results of fields. Only fields with a name containing all of the specified keywords (case insensitive) will be returned. | 
 **fruit_name_contains** | [**String**](String.md)| A set of keywords used to filter the results of fields. Only fields with a fruit_name containing all of the specified keywords (case insensitive) will be returned. | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Field>**](Field.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneField**
> models::Field getOneField(ctx, ctx, field_id)
Returns a specific stored Field from the collection

Returns a specific stored Field from the collection. The parameter `FieldId` is required, to get the specific stored Field. If the operation was successful (status code `200`), the specific Field will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 

### Return type

[**models::Field**](Field.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchOneField**
> models::Field patchOneField(ctx, ctx, field_id, field_patch)
Patches a specific stored Field

Patches a specific stored Field. The parameter `FieldId` is required, to replace the specific stored Field by the new one. Also the new data (`FieldPatch`) is required. If the operation was successful (status code `200`), the new Field will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field_id** | **i64**| The numeric ID of the field | 
  **field_patch** | [**FieldPatch**](FieldPatch.md)| The updated properties to patch | 

### Return type

[**models::Field**](Field.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postField**
> models::InlineResponse2011 postField(ctx, ctx, field)
Adds a new Field to the collection

Adds a new Field to the collection. In the request body you have to add the new Field, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Field will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **field** | [**Field**](Field.md)| A Field to be added to the collection | 

### Return type

[**models::InlineResponse2011**](inline_response_201_1.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

