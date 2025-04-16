# Checks

- [`e2e`](./e2e/README.md) covers logic for end-to-end validation of the migration.
- [`e2e`](./executor/README.md) covers logic for validating the 

All tests rely on clearly stated criteria APIs which are pulled up under a `Criterion` type. Before evaluating each `Criterion` a `Scenario` is run. Hence tests are formed:

```rust
test(
    Scenario,
    [
        CriterionA,
        CriterionB,
        CriterionC
    ]
)
```

The rigidity of this structure is to make it clear what the migration is accomplishing from the testing entrypoint. More standard and flexible assertions should be performed at lower levels in this repository. 

## `Criterion`
Tests are split into separate subdirectories which use their own `Criterion` types:

- `e2e` wherein a `Criterion` is evaluated against a `MovementRestClient` and a `MaptosRestClient` respectively. 
- `executor` wherein a `Criterion` is evaluated against a `MovementExecutor` and a `MaptosExecutor` respectively. 

