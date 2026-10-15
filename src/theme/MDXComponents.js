import MDXComponents from '@theme-original/MDXComponents';
import Example from '@site/src/components/Example';
import Screenshot from '@site/src/components/Screenshot';
import {Cards, Card} from '@site/src/components/Cards';

// Make these components available in every .mdx page without importing them.
export default {
  ...MDXComponents,
  Example,
  Screenshot,
  Cards,
  Card,
};
