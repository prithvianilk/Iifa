# whomping-willow

whomping-willow is a platform that allows you to create, edit and maintain complex conditional based actions and business rules in the form of decision trees.

## Features
- Create decision nodes supported by predicates that are evaluated at runtime.
- Create value nodes as final results of the tree evaluation.
- Link nodes to create powerful decision trees.
- You can provide input parameters used to evaluate the decision tree at runtime.
- Also, rust gang gang.

## whomping-willow as a platform
- User can CRUD on DTs.
- Each DT also has context, that can be updated or fetched externally from another service.
- This will allow for use cases, where we don't have context data stored but it is managed in another system.
- For a DT, a predicate should be constructed with an lhs and an rhs.
- `lhs` will be data received during evaulation via input parameters supplied.
- `rhs` will be constructed using context for the DT.
- `lhs` and `rhs` can be constructed using a serde_json's [pointer notation](https://docs.rs/serde_json/1.0.93/serde_json/enum.Value.html#method.pointer).
### Example: 
  input params: 
  ```
  { "monthly_income": 30000 }
  ``` 
  context: 
  ```
  {
	  "account_details": {
		  "min_monthly_income": 20000
	  }
  }
  ```
 You would create your predicate with paths as following.

 lhs: `/monthly_income`

 rhs: `/account_details/min_monthly_income`

## Predicates
- `LTNumber`: Checks if `lhs` < `rhs` (both are numbers)
- `GTNumber`: Checks if `lhs` > `rhs` (both are numbers)
- `EQNumber`: Checks if `lhs` = `rhs` (both are numbers)
- `EQString`: Checks if `lhs` = `rhs` (both are strings)
- `InListOfString`: Checks if `lhs` is in `rhs` (`lhs` is a string, `rhs` is a list of strings)

## An example decision tree for a bank application
<img width="1703" alt="Screenshot 2023-07-01 at 12 45 22 PM" src="https://github.com/prithvianilk/whomping-willow/assets/56789402/b49229a0-1f0a-4293-ad36-c8d57cf464a6">

## TODO
- [x] Make it non domain specific, use `json path`
- [x] Make it a platform? Allow creation of mutliple DTs, with context
- [ ] Add [a nice JSON editor](https://sujinleeme.github.io/react-json-editor/)
