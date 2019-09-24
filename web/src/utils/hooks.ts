import React from 'react';

export function useCrate() {
  const [wasm, setWasm] = React.useState();

  React.useEffect(() => {
    (async () => {
      const mod = await import('../crate');
      setWasm(mod);
    })();
  }, []);

  return wasm;
}

export function useTakeEffect(
  fn: () => void | (() => void),
  deps: React.DependencyList,
) {
  React.useEffect(() => {
    if (deps.some((d) => !d)) return;
    const destructor = fn();
    return () => {
      destructor && destructor();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, deps);
}
