# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**i2c_bus_api**](default_api.md#i2c_bus_api) | **GET** /i2c/api | 
**i2c_bus_list**](default_api.md#i2c_bus_list) | **GET** /i2c/buslist | 
**i2c_bus_read_byte**](default_api.md#i2c_bus_read_byte) | **GET** /i2c/{busId}/read/byte/{addr} | 
**i2c_bus_read_bytes**](default_api.md#i2c_bus_read_bytes) | **GET** /i2c/{busId}/read/bytes/{addr}/{numBytes} | 
**i2c_bus_read_reg**](default_api.md#i2c_bus_read_reg) | **GET** /i2c/{busId}/read/reg/{addr}/{reg}/{numBytes} | 
**i2c_bus_write_byte**](default_api.md#i2c_bus_write_byte) | **POST** /i2c/{busId}/write/byte/{addr}/{value} | 
**i2c_bus_write_byte_reg**](default_api.md#i2c_bus_write_byte_reg) | **POST** /i2c/{busId}/write/byte/reg/{addr}/{reg}/{value} | 
**i2c_bus_write_bytes**](default_api.md#i2c_bus_write_bytes) | **POST** /i2c/{busId}/write/bytes/{addr} | 
**i2c_bus_write_bytes_reg**](default_api.md#i2c_bus_write_bytes_reg) | **POST** /i2c/{busId}/write/bytes/reg/{addr}/{reg} | 


# **i2c_bus_api**
> String i2c_bus_api()


Returns this API specification

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, text/x-yaml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_list**
> Vec<models::I2cBusList> i2c_bus_list()


Returns the available I2C buses

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::I2cBusList>**](i2c_bus_list.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_read_byte**
> models::I2cBusRead i2c_bus_read_byte(bus_id, addr)


Reads a byte using the specified I2C slave address, using the specified bus.  Does not write the register to read from first

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to read from | 
  **addr** | **i32**| I2C slave address to read from | 

### Return type

[**models::I2cBusRead**](i2c_bus_read.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_read_bytes**
> models::I2cBusRead i2c_bus_read_bytes(bus_id, addr, num_bytes)


Reads a number of bytes using the specified I2C slave address, using the specified bus.  Does not write the register to read from first

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to read from | 
  **addr** | **i32**| I2C slave address to read from | 
  **num_bytes** | **i32**| Number of bytes to read | 

### Return type

[**models::I2cBusRead**](i2c_bus_read.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_read_reg**
> models::I2cBusRead i2c_bus_read_reg(bus_id, addr, reg, num_bytes)


Reads the specified number of bytes starting at the specified register (by writing that register to the bus before the read), using the specified I2C slave address, using the specified bus

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to read from | 
  **addr** | **i32**| I2C slave address to read from | 
  **reg** | **i32**| Register to read value from | 
  **num_bytes** | **i32**| Number of bytes to read | 

### Return type

[**models::I2cBusRead**](i2c_bus_read.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_write_byte**
> models::I2cBusOk i2c_bus_write_byte(bus_id, addr, value)


Writes the specified value, using the specified I2C slave address, using the specified bus.  Does not specify the register to write the value to

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to write to | 
  **addr** | **i32**| I2C slave address to direct this write to | 
  **value** | **i32**| Value to write to the register | 

### Return type

[**models::I2cBusOk**](i2c_bus_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_write_byte_reg**
> models::I2cBusOk i2c_bus_write_byte_reg(bus_id, addr, reg, value)


Writes the specified value to the specified register, using the specified I2C slave address, using the specified bus

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to write to | 
  **addr** | **i32**| I2C slave address to direct this write to | 
  **reg** | **i32**| Register to write value to | 
  **value** | **i32**| Value to write to the register | 

### Return type

[**models::I2cBusOk**](i2c_bus_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_write_bytes**
> models::I2cBusOk i2c_bus_write_bytes(bus_id, addr, values)


Writes the specified bytes using the specified I2C slave address, using the specified bus

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to write to | 
  **addr** | **i32**| I2C slave address to direct this write to | 
  **values** | [**Values**](Values.md)| Values to write | 

### Return type

[**models::I2cBusOk**](i2c_bus_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i2c_bus_write_bytes_reg**
> models::I2cBusOk i2c_bus_write_bytes_reg(bus_id, addr, reg, values)


Writes the specified bytes starting at the specified register, using the specified I2C slave address, using the specified bus

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus to write to | 
  **addr** | **i32**| I2C slave address to direct this write to | 
  **reg** | **i32**| Register to write value to | 
  **values** | [**Values**](Values.md)| Values to write | 

### Return type

[**models::I2cBusOk**](i2c_bus_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

