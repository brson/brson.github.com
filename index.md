---
layout: front
title: Front
---
{% include JB/setup %}

<section>
    {% for post in site.posts limit:5 %}
    <article>
        <div class="page-header">
            <h1>
                <a href="{{post.url}}">{{ post.title }}</a>
                <small>{{ post.date | date_to_string  }}</small>
            </h1>
        </div>

        {{ post.content }}
    {% endfor %}
    </article>
</section>
