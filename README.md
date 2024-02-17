# ai-coder
Automatically extends programming projects using language models agents.

## Idea
The development consists of this main loop.
1. Discover user intent
2. Create tests to validate the intent
3. Until the tests are satisfied, generate code
4. Create a merge request

### Discover user intent
In order to generate code according to some intent of the user, the agents have to understand first what is it that user actually wants.
This can be achieved fx. by generating a document describing the new update (similar to this one) and then asking the user if the document is aligned and sufficient.

### Create tests to validate the intent
As an additional layer of intent adherence validation serve tests which validate the code functionality. 
An agent generates these tests according to the readme and then gets them again approved or updated by the user.
