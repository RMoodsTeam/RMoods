# Frontend Styling Standards

## Core Configuration

### Preprocessor Setup
- **Primary**: Sass (`.scss`)
- **Secondary**: PostCSS
- **Utility Functions**: Available in `src/_mantine.scss`

## File Structure
```
component/  
├── ComponentName/  
│   ├── ComponentName.tsx  
│   └── ComponentName.module.scss
```
## Routes Structure
```
routes/  
├── routeName/  
│   └── page.tsx  
└── ui/  
  └── ComponentName/  
  ├── ComponentName.tsx  
  └── ComponentName.module.scss  
```

## Implementation Guidelines

### CSS Modules
```typescript jsx
// ComponentName.tsx
import classes from './ComponentName.module.scss';

export function ComponentName() {
  return <Box className={classes.container}>...</Box>;
}
// ComponentName.module.scss
.container {
// styles
}
```
### Class Naming
```css
// ✅ Correct
.container {}
.buttonPrimary {}
.navItem {}

// ❌ Incorrect
.Container {}
.button-primary {}
.nav_item {}
```

### Styling Hierarchy

- CSS Modules (Preferred)
- Mantine Props (For basic properties or non-obvious styling)
- Inline Styles (Prohibited)

### Mantine Usage
```typescript jsx
// ✅ Correct (only if the prop has a specified name in the component's documentation)
<Button gap='md'/>

// ❌ Incorrect
<Button styles={{ root: { backgroundColor: 'blue' }}} />
```

## Technical Requirements

### File Extensions
Every style file should end with `.module.scss`

### Import Conventions
```
// Every styles import should be named 'classes'
import classes from './ComponentName.module.scss';
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