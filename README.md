# Challenge Statement

This challenge is about creating a simple video storage server with REST APIs

## Details

You are tasked to develop a simple video storage server with REST APIs, which should have:
- **CRUD implemention** as described in the [Open API definition](./api.yaml). (This document only contains the minimum and may need to be added).
- **Dockerfile** and **docker-compose** to build and run the server and other required services as docker containers.
- The endpoints of the server are exposed to the host machine.

## What we expect

When working on this challenge, be sure to:

- prove correctness of the code. We don't expect 100% test coverage but highlights on critical paths and logic is very welcome.
  
- handle errors and bad inputs
  
- provide user friendliness of installation and setup. We'll run `docker-compose up` in a clean environment without toolchains, JVM or SDKs and expect to see a server and the needed containers building and starting (this includes DB and all the other images used to complete the task).

We understand taking a challenge is time consuming, so feel free to choose an additional feature you feel passionate about and explain in words how you would like to implement it. We can discuss it further during the next interview steps!

## How to submit your solution

- Push your code to this repository in the `main` branch.
- Make sure the endpoints follow the path suggested in the `api.yaml` file (v1 included!).
- If your setup is correct the CI will return a green check and you can move forward. 

⚠️ **Note #1**: the CI/CD runs checks against a set of tests necessary to consider the assigment correct. _Without a green check we won't review the challenge_ as we can safely assume the overall solution is incomplete. Also, please *DO NOT* change the CI/CD workflow file _or_ the `test/tester.json` file - if you want to add your own tests, please add them in a dedicated folder of your choice.

⚠️ **Note #2**: if you add or change APIs, include its OpenAPI document. However, please note that your server may be accessed by external clients in accordance with the given OpenAPI document and automated tests will hit the endpoints as described in [api.yaml](./api.yaml), therefore any change in the base path could result in 404 or false negative.

## FAQ
_Questions outside of this FAQ will not be answered. Please include them with your challenge submission and they can be covered in the technical interview stage._


**My submission is not passing the health check. Can I still submit it for review?**

Your submission must pass all automated tests to be considered for review. If it doesn’t pass, your solution won’t be considered correct. The workflow should also pass without modification. 

**Do you have any limit to the file size?**

No. As long as the CI pipeline doesn’t complain we don’t have requirements.

**Do I need to demonstrate other aspects in backend developments, like code quality, to pass because 100% test coverage is the basic requirement?**

Code quality, folder structure and software development principles are considered during the solution review. Naming variables “xyz” is usually a minus even if the solution is correct.

**Where should I store the files? File system or database?**

That’s up to you. A local storage is preferred to a remote one (S3 needs keys for example), but there’s no strong preference between file system and database as long as the API requirements are satisfied. 

**Is any language/framework fine as long as the 'Expectations' are met?**

You can use any language. 

**Is storage meant to be local, cloud-based, or either?**

That’s up to you. Local storage is preferred to a remote one unless you are comfortable with committing keys to your cloud storage (we suggest avoiding this and we are not responsible for any leak of your credentials), but there’s no strong preference between file system and database as long as the API requirements are satisfied.

**Is Unit testing also required?**

It isn't required, but it is nice to have.

**OpenAPI has been used for api specification, is it ok if I use Swagger UI to visualize openapi yaml?**

Yes, it is OK to use swagger UI as long as it doesn't interfere with the main service.

**Do I only have one chance to push to the main branch (or ext-solution branch) of the repository?**

No. That’s your repository and your main branch, we just ask you to keep your commits and PR tidy. If you feel like branching you can also do this, just tell us to check the branch to evaluate your thought process better.



Again, we appreciate you taking the time to work on this challenge and we are looking forward to your submission!

