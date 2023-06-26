# whomping-willow

- whomping-willow is a platform that allows you to create, edit and maintain complex conditional based actions and business rules in the form of decision trees.
- You can create decision nodes supported by predicates that are evaluated at runtime.
- You can create value nodes as final results of the tree evaluation.
- You can provide input parameters and can fetch decision tree context from a DB / external system. Then, you can use these values to evaluate your predicate at runtime.
- The project is a wip rn.
- Also, rust gang gang.

### Decision Tree for a bank application
<img width="1717" alt="Screenshot 2023-06-26 at 4 29 12 AM" src="https://github.com/prithvianilk/whomping-willow/assets/56789402/b0ef695c-b00c-431b-a6cf-7314049195c2">

# whomping-willow as a platform
- User can CRUD on DTs
- Each DT also has context, that can be updated or fetched externally
- This will allow for use cases, where we don't have context data stored but it is stored in another system
- For a DT, a predicate should be constructed with an lhs and an rhs
- lhs will be data received during evaulation via user input
- rhs will be constructed using context for the DT
- lhs and rhs can be constructed using a serde_json's [pointer notation](https://docs.rs/serde_json/1.0.93/serde_json/enum.Value.html#method.pointer).
- Example: 
  - input: ```{ "monthly_income": 10000 }```, 
  - context: ```{ "min_monthly_income": 20000 }```
  - lhs: `/monthly_income`, rhs: `/minimum_monthly_income`

# TODO
- [x] Make it non domain specific, use `json path`
- [ ] Make it a platform? Allow creation of mutliple DTs, with context
