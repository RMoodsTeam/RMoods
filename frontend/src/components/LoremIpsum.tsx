interface LoremIpsumProps {
    n: number;
  }
  
  const LoremIpsum = ({ n }: LoremIpsumProps) => {
    const loremText = `Lorem ipsum odor amet, consectetuer adipiscing elit. Praesent in dignissim odio blandit lectus.
      Erat aliquam sollicitudin suscipit, magnis vitae dapibus ex venenatis. Mus libero sodales amet eget mi augue.
      Facilisis magnis venenatis; quis viverra per habitasse quisque. Turpis venenatis blandit tempor condimentum
      neque volutpat sapien mollis. Dignissim eleifend vel aliquam at turpis cubilia quam sem ad. Himenaeos sit
      ullamcorper tempor sagittis fames.`;
  
    return (
      <div>
        {Array.from({ length: n }, (_, i) => (
          <p key={i}>{loremText}</p>
        ))}
      </div>
    );
  };
  
  export default LoremIpsum;