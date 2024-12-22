# Styling standards
This file contains the styling  standards for the project's frontend.

## CSS preprocessor and postprocessor
We have to remember that we use preprocessor and postprocessor (Sass and PostCSS). That means that module files should end in `.scss` and not `.css`. We also have a few predefined functions (src/_mantine.scss)

## No inline styles
Using `styles` prop is not allowed. Any CSS style that is applied to a component should be a defined prop in the component's documentation. See next section for how to style components.

## Module CSS
Any CSS written should be in a module file next to the `.tsx` file. Those two files should be contained in a directory with the same name so an example structure would be:
```
src/
  components/
    Button/
      Button.tsx
      Button.module.scss
```

## Naming conventions
As we are using CSS modules, we don't need to worry about class name collisions. We should stick to KISS principle and use simple class names. Names should be a single word, lowercase and using camelCase in case of multiple words. For example:
```css
.button {
  background-color: red;
}

.buttonPrimary {
  background-color: blue;
}
```

## Importing CSS
When importing `.module.scss` file in a `.tsx` file, the import should look like this:
```tsx
import classes from './Button.module.scss';
```

## Structure in `/routes` direcrory
In `/routes` directory components that are related to the route but are not the page itself have to be in a `/ui` directory. For example:
```
src/
  routes/
    login/
      page.tsx
    ui/
      Button/
      Button.tsx
      Button.module.scss
```

