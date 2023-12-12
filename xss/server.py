from flask import Flask, request, render_template_string
from markupsafe import Markup

app = Flask(__name__)
messages = []


@app.route("/")
def index():
    return render_template_string(
        """
    <html>
<head>
    <title>Simple Web Chat</title>
    <link rel="stylesheet" type="text/css" href="{{ url_for('static', filename='styles.css') }}">
</head>

<body>
    <h1>Simple Web Chat</h1>
    <h3>Style your messages using power of html!</h3>
    <div id="chat">
        <ul>
            {% for message in messages %}
            {{ message }}
            {% endfor %}
        </ul>
    </div>
    <form action="/send" method="post">
        <input type="text" name="message" placeholder="Type your message" required>
        <button type="submit">Send</button>
    </form>
</body>
</html>
    """,
        messages=list(map(lambda x: Markup("<li>" + x + "</li>"), messages)),
    )


@app.route("/send", methods=["POST"])
def send():
    message = request.form["message"]
    messages.append(message)
    return """
    <html>
    <head>
        <meta http-equiv="refresh" content="0; URL='/'" />
    </head>
    <body>
        <p>Message sent. Redirecting back to chat...</p>
    </body>
    </html>
    """


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
