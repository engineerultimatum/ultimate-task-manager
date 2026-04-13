# Acceptance Test and Acceptance Criteria
- Test Name: Load data using seed
- Type: Functional
- Input: Seed = 12345
- Expected Result: System loads the dataset corresponding to seed 12345 Output is consistent across multiple runs
---
- Test Name: App Launch
- Expectation for funcitonal part: App launches successfully
- Expectation for non-functional part: App is ready to use under 15 seconds from launch command
- Test Environment: The application will be cold-started on a Windows 11 system under normal operating conditions
- Test Procedure: The test will be executed three times under identical conditions.
  The execution time will be recorded for each run, and the average value will be used as the final result
---
- Test Name: Adding new tasks
- Expectation for funcitonal part: A new task is added successfully
- Expectation for non-functional part: The new task is visible within less than a second

---

- Test Name: Calendar functionality
- Type: Functional
- Expectation: A new event to a future calendar day can be added/removed
