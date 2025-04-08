# Frontend Styling Standards

## Core Configuration

### Preprocessor Setup

- **Primary**: Sass (`.scss`)
- **Secondary**: PostCSS
- **Utility Functions**: Available in `src/_mantine.scss`

## File Structure

In general, a component's implementation is structured like so:

```
ComponentName/  
├── ComponentName.tsx  
├── ComponentName.module.scss
└── ComponentFallback.tsx
```

It contains a `.tsx` file along with `.module.scss` for styling, all inside a directory named like the component in
question, but using camelCase.
In some cases, it also contains a fallback component in case an error is thrown while rendering.

## Routes Structure

When some components are needed to implement a route and are not used just in one place, you can separate them out to
the `ui` directory inside the route directory.

```
routes/  
└── routeName/  
    ├── page.tsx  
    └── ui/  
        └── ComponentName/  
            ├── ComponentName.tsx  
            └── ComponentName.module.scss  
```

## Implementation Guidelines

### SCSS Modules

SCSS Modules are used to scope styles locally by default, preventing style conflicts:

```typescript jsx
// ComponentName.tsx
import classes from './ComponentName.module.scss';

export function ComponentName() {
  return <Box className={classes.container}>...</Box>;
}


// ComponentName.module.scss
.
container
{
  // styles
}
```

### Class Naming

All classes must be named in camelCase

```css
✅ Correct
.container {
}

.buttonPrimary {
}

.navItem {
}


❌ Incorrect
.Container {
}

.button-primary {
}

.nav_item {
}
```

### Mantine Usage

Mantine is our framework of choice, and it contains some useful props. However only the props listed in the component's
Props section in Mantine docs can be used.
For example: https://mantine.dev/core/textarea/?t=props

All props listed there can be used. That leaves props such as `w`, `h`, `flex` etc. as prohibited, as they can be easily
emulated using SCSS.

```typescript jsx
// ✅ Correct (only if the prop has a specified name in the component's documentation)
<Button gap='md' />

// ❌ Incorrect
<Button styles={{ root: { backgroundColor: 'blue' } }} />
```

### Styling Hierarchy

- Mantine Props (Only if listed in docs)
- SCSS Modules (Preferred)
- Inline Styles (Prohibited)

## Technical Requirements

### File Extensions

Every style file should end with `.module.scss`

### Import Conventions

Every styles import should be named 'classes'

```
// ✅ Correct 
import classes from './ComponentName.module.scss';

// ❌ Incorrect
import styles from './ComponentName.module.scss';
```

### Module Resolution

- All style modules must be typed
- Use relative paths for component-specific imports
- Use absolute paths for shared resources

## Prohibited Practices

- Direct usage of .css files
- Inline styles via style prop
- Non-modular CSS
- Global styles (except for root-level configuration)
- Style prop usage in Mantine components