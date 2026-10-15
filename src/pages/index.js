import React from 'react';
import {Redirect} from '@docusaurus/router';
import useBaseUrl from '@docusaurus/useBaseUrl';

// The docs are the site: send visitors straight to the introduction.
export default function Home() {
  return <Redirect to={useBaseUrl('/docs/')} />;
}
