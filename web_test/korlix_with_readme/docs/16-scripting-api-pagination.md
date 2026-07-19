# Scripting, API Calls and Pagination

## Daily-use scripting

```klx
page Tasks at "/tasks"
  state tasks = []
  state filter = "all"

  if tasks.length == 0
    empty-state title="No tasks"
  else
    for task in tasks
      card
        p task.title

  fn addTask(title: string)
    tasks = list.add(tasks, { title: title })
```

Supported expression and statement features include strings, numbers, booleans, null, lists, records, arithmetic, comparison, boolean operators (`and`, `or`, `not`), member access, index access, calls, conditions, loops, functions, typed parameters, assignments and compound assignments.

Events can reference a function or use an inline block:

```klx
button "Save" click=save
button "Increase" click
  count += 1
```

## API queries

```klx
page Users at "/users"
  get users "/api/users"

  if users.loading
    spinner
  else
    for user in users.data
      profile-card user=user
```

The generated query state provides `data`, `loading`, `error` and reload support through the Korlix runtime.

## Mutations

```klx
fn saveUser
  post "/api/users" { name: name, email: email }
  reload users
```

Supported methods are `get`, `post`, `put`, `patch` and `delete`. Member-call form is also generated for `api.get(...)`, `api.post(...)`, `api.put(...)`, `api.patch(...)` and `api.delete(...)` inside functions.

## Pagination

```klx
state page: int = 1
pagination page=page pages=12 url-sync
```

Or use total records and page size:

```klx
pagination page=page total=products.total size=20 url-sync
```

The runtime renders first, previous, numbered, next and last controls, dispatches a change event and can synchronize the current page to the URL query string.
