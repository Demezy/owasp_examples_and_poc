from flask import Flask, request, render_template_string
from markupsafe import Markup
import sqlite3

app = Flask(__name__)


@app.route("/")
def index():
    # Retrieve todos from the database
    with get_conn() as conn:
        cursor = conn.cursor()
        cursor.execute("SELECT title, content FROM todos")
        todos = [row[0:2] for row in cursor.fetchall()]

    return render_template_string(
        """
    <html>
<head>
    <title>Todo app!</title>
</head>

<body>
    <h1>Todo app!</h1>
    <h3>Store your tasks securely</h3>
    <div id="todos">
        <ul>
            {% for todo in todos %}
            {{ todo}}
            {% endfor %}
        </ul>
    </div>
    <form action="/send" method="post">
        <input type="text" name="title" placeholder="title" required>
        <input type="text" name="content" placeholder="content" required>
        <button type="submit">Send</button>
    </form>
    <a href="/query">Query by title</a>

</body>
</html>
    """,
        todos=list(
            map(
                lambda x: Markup("<li>" + x[0] + " | " + x[1] + "</li>"),
                todos,
            )
        ),
    )


@app.route("/send", methods=["POST"])
def send():
    form = request.form
    print(form)
    with get_conn() as conn:
        cursor = conn.cursor()
        cursor.execute(
            "INSERT INTO todos (title, content) VALUES (?, ? )",
            (
                form["title"],
                form["content"],
            ),
        )
        conn.commit()
    return """
    <html>
    <head>
        <meta http-equiv="refresh" content="0; URL='/'" />
    </head>
    <body>
        <p>todo sent. Redirecting back to task list...</p>
    </body>
    </html>
    """


@app.route("/query", methods=["GET"])
def query_page():
    title = request.args.get("title")
    return render_template_string(
        """
    <html>
<head>
    <title>Todo app!</title>
</head>

<body>
    <h1>query data</h1>
    <form action="/query" method="get" onsubmit="event.preventDefault(); navigateToQuery()">
        <input type="text" name="title" id="titleInput" placeholder="search by title" required>
        <button type="submit">Send</button>
    </form>
    <h3>search result</h3>
    <div id="todos">
        <ul>
            {% for todo in todos %}
            {{ todo }}
            {% endfor %}
        </ul>
    </div>
    <a href="/">home</a>

    <script>
        function navigateToQuery() {
            const title = document.getElementById("titleInput").value;
            window.location.href = `/query?title=${encodeURIComponent(title)}`;
        }
    </script>
</body>
</html>
    """,
        todos=list(
            map(
                lambda x: Markup("<li>" + " | ".join(map(str, x)) + "</li>"),
                query_by_title(title),
            )
        ),
    )


def query_by_title(title):
    if not title:
        return []
    with get_conn() as conn:
        query = f"SELECT title, content FROM todos WHERE title = '{title}'"
        print()
        print(query)
        print()
        cursor = conn.cursor()
        # VULNERABLE CODE! User has full control over the query
        cursor.execute(query)
        todos = cursor.fetchall()
    return todos


def get_conn():
    conn = sqlite3.connect("todos.db")
    return conn


if __name__ == "__main__":
    # connect db
    with get_conn() as conn:
        # Create the todos table if it doesn't exist
        conn.cursor().execute(
            """
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                content TEXT
            )
        """
        )

    app.run(host="0.0.0.0", port=5000)
