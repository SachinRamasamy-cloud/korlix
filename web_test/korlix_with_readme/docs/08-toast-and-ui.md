# Toast, Modal & UI Interactions

## Toast System

### Basic Usage

```klx
# Inside any event handler:
btn "Save" on:click:
  toast success "Saved successfully"
  toast error "Failed to save"
  toast warning "Check your input"
  toast info "Processing..."
  toast loading "Uploading..."
```

### With Options

```klx
btn "Notify" on:click:
  toast success "Done!" position="bottom-right" duration=5000
```

Props:
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| type | string | "info" | success \| error \| warning \| info \| loading |
| message | string | required | Toast text |
| duration | number | 3000 | Auto-dismiss (ms). 0 = manual only |
| position | string | "top-right" | top-right \| top-left \| top-center \| bottom-right \| bottom-left \| bottom-center |

### Via JS Runtime

```js
KorlixRuntime.Toast.show('success', 'Saved!', { duration: 4000 });
```

## Modal System

### Basic Modal

```klx
btn "Open" on:click:
  openModal("terms")

modal id="terms" title="Terms of Service":
  p "These are the terms..."
  slot:footer:
    btn .primary "Accept" on:click:
      closeModal("terms")
    btn .ghost "Cancel" on:click:
      closeModal("terms")
```

### Confirm Dialog

```klx
btn .danger "Delete" on:click:
  openModal("delete-confirm")

modal id="delete-confirm" title="Delete Item?" size="sm":
  p "This action cannot be undone."
  slot:footer:
    btn .danger "Yes, Delete" on:click:
      deleteItem()
      closeModal("delete-confirm")
    btn .ghost "Cancel" on:click:
      closeModal("delete-confirm")
```

### Programmatic

```klx
action handleDelete:
  openModal("confirm")

action onConfirm:
  deleteItem(currentId)
  closeModal("confirm")
  toast success "Deleted"
  navigate("/list")
```

## Drawer

```klx
btn "Settings" on:click:
  openDrawer("settings")

drawer id="settings" title="Settings" side="right":
  div .p-6:
    h3 .font-semibold .mb-4 "Preferences"
    form-field:
      switch label="Dark mode" checked=true
    form-field:
      switch label="Notifications" checked=false
```

## UI Functions Reference

```text
toast(type, message, opts?)     Show a toast notification
openModal(id)                   Open a modal dialog
closeModal(id)                  Close a modal dialog
openDrawer(id)                  Open a side drawer
closeDrawer(id)                 Close a side drawer
navigate(path)                  SPA navigate to path
goBack()                        Browser history back
toggleTheme()                   Switch dark/light mode
scrollTo(selector)              Smooth scroll to element
copyToClipboard(value)          Copy text to clipboard
log(value)                      Console log (debug)
```
