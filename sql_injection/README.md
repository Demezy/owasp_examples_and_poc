access 0.0.0.0:8002 (if deploy in docker compose)

service for maintainig todo lists. Since Author is procrastinator we only add tasks and not complete them

here is link that show all entries in database. in case of our procrastinator it is not that ctitical, but for bigger system with access control...? 
Do you want your tasks to be available?
http://127.0.0.1:5000/query?title=%27%20or%201%3D%3D1%20or%20%27%27%3D%27

<!-- http://127.0.0.1:5000/query?title=1%27%20union%20all%20select%20(%27test%27%2C%20sqlean_fileio(%27super-secret-file.txt%27))%20where%201=1%20or%20%27%27%3D%27; -->

since sercice does not escape sql query there is full access to sql power. SImplest example is drop all tables. But as more damaging one is access to provate files of the user (if deployment is not isolated) (google sql load file) or network queries (check it, not sure). Fortunately default config prevent mutiple instructions per execution, but still, other sql provider and thats it => no data or some worse things

how to fix:
- use orm (raw requests are dangerous)
- escape strings
- isolate workers
- limit service from network, since it only handle io and transformations 
