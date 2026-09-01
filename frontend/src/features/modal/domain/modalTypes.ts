export type ConfirmationVariant = 'destructive' | 'warning' | 'info';

export type ConfirmationOptions = {
  title: string;
  description?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: ConfirmationVariant;
};

export type ResolvedConfirmationOptions = {
  title: string;
  description?: string;
  confirmLabel: string;
  cancelLabel: string;
  variant: ConfirmationVariant;
};
