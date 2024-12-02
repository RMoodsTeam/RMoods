import { Affix, Button, Transition, rem } from '@mantine/core';
import { IconArrowUp } from '@tabler/icons-react';
import { useWindowScroll } from '@mantine/hooks';

export const ScrollToTop = () => {
  const [scroll, scrollTo] = useWindowScroll();

  return (
    <Affix position={{ bottom: rem(80), right: rem(20) }}>
      <Transition transition="slide-up" mounted={scroll.y > 0}>
        {(transitionStyles) => (
          <Button
            style={transitionStyles}
            onClick={() => scrollTo({ y: 0 })}
            variant="light"
            radius="xl"
            aria-label="Scroll to top"
          >
            <IconArrowUp size="1.2rem" />
          </Button>
        )}
      </Transition>
    </Affix>
  );
};
