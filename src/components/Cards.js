import React from 'react';
import Link from '@docusaurus/Link';

/** A grid of link cards, used on the gallery pages. */
export function Cards({children}) {
  return <div className="gallery">{children}</div>;
}

/** One card: `<Card to="/docs/widgets/label" title="Label">What it is for</Card>` */
export function Card({to, title, children}) {
  return (
    <Link to={to}>
      <strong>{title}</strong>
      <span>{children}</span>
    </Link>
  );
}
