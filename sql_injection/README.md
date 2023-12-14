# SQL Injection

The following service is for maintaining TODO lists. Since the author is a
procrastinator we can only add tasks but not complete them.

By default the service is available at `localhost:8002`

## Exploit


Here is link that shows all entries in database. In case of our procrastinator it
is not that critical, but for a bigger system with access control it it. Do you
want your tasks to be available?

http://127.0.0.1:5000/query?title=%27%20or%201%3D%3D1%20or%20%27%27%3D%27

<!-- http://127.0.0.1:5000/query?title=1%27%20union%20all%20select%20(%27test%27%2C%20sqlean_fileio(%27super-secret-file.txt%27))%20where%201=1%20or%20%27%27%3D%27; -->

Failure to escape SQL queries can result in significant security risks,
providing unrestricted access to the full power of SQL. For instance, it can
lead to destructive actions such as dropping all tables, accessing private files
(if deployment is not isolated), or executing network queries. While the default
configuration prevents multiple instructions per execution, other SQL providers
may not offer the same level of protection, potentially resulting in data loss
or even more severe consequences

## Fixes

- Use an ORM to interact with the database, as raw SQL requests can be risky
- Always escape strings to prevent SQL injection attacks
- Isolate workers to improve system robustness and security
- Limit the responsibilities of a service to handling I/O and transformations, especially when it interacts with the network

## References

- [Definition at owasp.org](https://owasp.org/www-community/attacks/SQL_Injection)
