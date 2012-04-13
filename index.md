---
layout: page
title: Front
---
{% include JB/setup %}

<style type="text/css">
.page-header { display: none }
</style>

<section>
    {% for post in site.posts limit:5 %}
    <article>
        <h1>
            <a href="{{post.url}}">{{ post.title }}</a>
            <small>{{ post.date | date_to_string }}</small>
        </h1>
        {{ post.content }}
    </article>
    {% endfor %}
</section>
