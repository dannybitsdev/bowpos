import { useModalStore } from '../application/modalStore';
import type { ConfirmationOptions } from '../domain/modalTypes';

/**
 * Imperative replacement for `window.confirm`. Usage:
 * `const confirmed = await confirm({ title, description, variant });`
 */
export function useConfirmationModal() {
  const request = useModalStore((state) => state.request);

  function confirm(options: ConfirmationOptions): Promise<boolean> {
    return request(options);
  }

  return { confirm };
}
