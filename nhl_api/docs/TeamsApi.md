# \TeamsApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_team**](TeamsApi.md#get_team) | **GET** /teams/{id} | Get an NHL team.
[**get_team_roster**](TeamsApi.md#get_team_roster) | **GET** /teams/{id}/roster | Get an NHL team's roster.
[**get_team_stats**](TeamsApi.md#get_team_stats) | **GET** /teams/{id}/stats | Get all statistics for an NHL team.
[**get_teams**](TeamsApi.md#get_teams) | **GET** /teams | Get all NHL teams.



## get_team

> crate::models::Team get_team(id, expand, season)
Get an NHL team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | [required] |
**expand** | Option<**String**> | Expand your response for some additional data. |  |
**season** | Option<**f32**> | Return a team's specific season. |  |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_roster

> crate::models::Rosters get_team_roster(id, season)
Get an NHL team's roster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | [required] |
**season** | Option<**f32**> | Return a team's specific season. |  |

### Return type

[**crate::models::Rosters**](Rosters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_stats

> crate::models::TeamStats get_team_stats(id)
Get all statistics for an NHL team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | [required] |

### Return type

[**crate::models::TeamStats**](TeamStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> crate::models::Teams get_teams(expand, season)
Get all NHL teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand your response for some additional data. |  |
**season** | Option<**f32**> | Return a team's specific season. |  |

### Return type

[**crate::models::Teams**](Teams.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

