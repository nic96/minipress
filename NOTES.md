#### Routes
* `/` - Shows a list of blog posts in chronological order
* `/YYYY/MM/DD/slug` - The individual posts
* `/login` - Post requests with the current route
  as a request param. Will redirect to github oauth.
* `/auth/{redirect_url}` - Callback url for github oauth.

#### Post format
Post will be written in markdown and parsed using
`pulldown-cmark` crate.
