# iifa

- Experimenting with decision trees for creating, editing and maintaining complex conditionals and business rules.
- You can create decision nodes supported by predicates that are evaluated at runtime.
- You can create value nodes as final results of the tree evaluation. 
- Also, rust gang gang.

<img width="1387" alt="Screenshot 2023-06-19 at 2 41 50 PM" src="https://github.com/prithvianilk/Iifa/assets/56789402/f049ee14-0579-403f-b624-cb6ade7ca27d">
 
# iifa as a platform
- User can CRUD on DTs
- Each DT also has context, that can be updated or fetched externally
- This will allow for use cases, where we don't have context data stored but it is stored in another system
- For a DT, a predicate should be constructed with an lhs and an rhs
- lhs will be data received during evaulation via user input
- rhs will be constructed using context for the DT
- Example: 
  - input: { "monthly_income": 10,000 }. context: { "min_monthly_income": 20,000 }
  - lhs: monthly income, rhs: minimum monthly income
  - lhs and rhs can be constructed using a json path

# TODO
- [x] Make it non domain specific, use `json path`
- [ ] Make it a platform? Allow creation of mutliple DTs, with context
