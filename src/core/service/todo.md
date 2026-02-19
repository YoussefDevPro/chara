so uh, i finished working on the models (well, still going to work on it lol)
now i gotta work on the second most important thing ever, SERVICES....
so yeah, services is just functions to interact with the db, u give him the request, they give u the data result OR an error 
the first step would be to make the * thats it time to play with my babe ill finish this later :3 * 

alr, gonna rework on this again lol, so now we have the models, the gud thing abt surrealdb, is that we can make a diff db/namespace to store info like notifications and stuff , and let the important stuff in the main db, (second db for like, preferences etc etc)
 and uh, my uh VERY PRIVATE SERVER is going to be my test to store those kind of stuff 
 alr, now whats the plan, the important part in our service is user, so we better start with user to get the user using sessions tokens or hackclub auth ids
 sessions and hcauth stuff will be inside the user service, and we secure everything by just getting the stuff we are supposed to get /silly
 like, raw session string and uh (reads hackclub auth docs) by getting the token we use to do the request, and uh, not sure if its a gud idea to do the request localy in the server, but we must make the hcauth api secure for this kind of requests
