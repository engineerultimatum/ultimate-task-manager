# Delta Desgin & Implementations
Before: The project was mainly built on monolithic architecture. This allowed fast development and prototyping but caused increasingly complex structure as new functionalities were added.
Design Decision: After discussion with Auditor Group and Product Assurance Group, we decided to transition to layered architecture.
## Cost Estimations
As code base grew Monolithic architecture became harder to extend and maintain. Cost estimations yielded increasingly higher results for adding new feature and developers were having difficulty understanding code.
## Justification of Layered Architecture
Layered architecture provides a structured way by dividing the application into layers. By dividing the application into layers and further dividing core functions into their own files, code complexity
can be significantly reduced.
## Effects
After implementing layered architecture, number of files significantly increased and this intrduced a file organization issue. But in return of this, software became much more maintainable.
Short files with one or two functions enable us to pinpoint exact issues and modifying components with less dependency conflicts.
