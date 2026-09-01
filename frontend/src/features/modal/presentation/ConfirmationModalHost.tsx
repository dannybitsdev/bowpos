import { useModalStore } from '../application/modalStore';
import { ConfirmationModal } from './ConfirmationModal';

/** Container: mount once at the app root; renders the modal driven by `useModalStore`. */
export function ConfirmationModalHost() {
  const isOpen = useModalStore((state) => state.isOpen);
  const options = useModalStore((state) => state.options);
  const confirm = useModalStore((state) => state.confirm);
  const cancel = useModalStore((state) => state.cancel);

  if (!isOpen || !options) return null;

  return (
    <ConfirmationModal
      open={isOpen}
      title={options.title}
      description={options.description}
      confirmLabel={options.confirmLabel}
      cancelLabel={options.cancelLabel}
      variant={options.variant}
      onConfirm={confirm}
      onCancel={cancel}
    />
  );
}
