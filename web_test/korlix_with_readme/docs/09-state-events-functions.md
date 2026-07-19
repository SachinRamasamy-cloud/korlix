# State, Events & Functions

## State

### Declaration

```klx
state count: int = 0
state name: string = "World"
state open: bool = false
state items: list<string> = []
state user: record = { name: "Sachin", role: "admin" }
```

### Data Types

```text
string      "hello"
int         42
float       3.14
number      42 or 3.14
bool        true / false
null        null
list<T>     [1, 2, 3]
record      { key: value }
```

### Reactive Binding

State automatically updates the DOM when changed:

```klx
state count: int = 0

p count              # Displays current count
p "Count: " count    # Inline binding
```

### Let & Derived

```klx
# Let: local constant (not reactive)
let title = "Korlix Dashboard"

# Derived: computed from state
derived total = price * quantity
derived isValid = name.length > 0 && email.length > 0
derived greeting = "Hello, " + user.name
```

## Events

### Basic Events

```klx
btn "Click me" on:click:
  count = count + 1
  toast success "Clicked!"

input placeholder="Name" on:input:
  name = event.target.value

form on:submit:
  submitForm(name, email)
```

### Multiple Actions in Handler

```klx
btn .primary "Save & Close" on:click:
  saveData()
  closeModal("form")
  toast success "Saved!"
  navigate("/dashboard")
```

### Available Events

```text
on:click          on:dblclick       on:mouseenter     on:mouseleave
on:mouseover      on:mouseout       on:mousedown      on:mouseup
on:keydown        on:keyup          on:keypress
on:input          on:change         on:focus          on:blur
on:submit         on:reset
on:scroll         on:resize
on:mount          on:unmount
```

## Actions (Named Functions)

```klx
action fetchUsers:
  data users = get "/api/users"
  toast info "Users loaded"

action deleteItem (id: string):
  delete "/api/items/" + id
  toast success "Deleted"
  navigate("/items")

action handleSubmit:
  if name.length < 2:
    toast error "Name too short"
  else:
    post "/api/contact" { name: name, email: email }
    toast success "Message sent!"
    navigate("/")
```

## Built-in Functions

### String

```klx
upper(name)             # "SACHIN"
lower(name)             # "sachin"
trim(input)             # Remove whitespace
contains(text, "hello") # bool
startsWith(url, "https")
endsWith(file, ".klx")
replace(text, "old", "new")
slug(title)             # "my-title"
capitalize(word)        # "Korlix"
```

### Number

```klx
round(3.7)              # 4
floor(3.7)              # 3
ceil(3.2)               # 4
min(a, b)
max(a, b)
clamp(value, 0, 100)
formatNumber(1000000)   # "1,000,000"
formatCurrency(9.99, "USD")  # "$9.99"
```

### List

```klx
count(items)
isEmpty(items)
filter(items, fn)
map(items, fn)
find(items, fn)
sort(items)
reverse(items)
slice(items, 0, 5)
take(items, 3)
unique(items)
```

### Date

```klx
now()
formatDate(date, "YYYY-MM-DD")
formatTime(date, "HH:mm")
addDays(date, 7)
diffDays(startDate, endDate)
isToday(date)
```

### Validation

```klx
isEmpty(value)
isEmail(value)
isUrl(value)
```

## Conditional Logic

```klx
if isLoggedIn:
  btn "Dashboard" on:click:
    navigate("/dashboard")
else:
  btn "Login" on:click:
    navigate("/login")

if count > 10 && isAdmin:
  div .admin-panel:
    p "Admin controls"
```

## Loops

```klx
# Basic loop
for item in items:
  li item.name

# With index (use derived)
for product in products:
  product-card
    name=product.name
    price=product.price
    on:click:
      navigate("/products/" + product.id)
```
