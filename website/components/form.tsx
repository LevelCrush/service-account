import React from 'react';
import Container from '@website/components/elements/container';
import { H3 } from '@website/components/elements/headings';

/**
 * Default style for label elements
 */
const STYLE_LABEL = 'inline-block text-lg hover:cursor-pointer ';

export interface FormFieldPropsOption {
  value: string;
  text?: string;
}

/**
 * Form Field properties that are mandatory/optional. Additionally extends more html attributes for future usage
 */
export interface FormFieldProps extends React.HTMLAttributes<HTMLElement> {
  label: string;
  name: string;
  id: string;
  maxLength?: number;
  required?: boolean;
  type:
    | React.HTMLInputTypeAttribute
    | 'custom'
    | 'select'
    | 'textarea'
    | 'toggle';
  placeholder?: string;
  value?: string;
  options?: FormFieldPropsOption[];
  disabled?: boolean;
  list?: string;
  textarea?: {
    rows?: number;
    columns?: number;
  };
}

/**
 * Renders a generic form field
 * @param props
 * @returns
 */
function render_default(props: FormFieldProps) {
  return (
    <>
      <label className={STYLE_LABEL} htmlFor={props.id}>
        {props.label}
      </label>
      <input
        className="block text-base p-[.25rem] text-black border-black border-[1px] w-full disabled:bg-gray-400 h-10 rounded-none"
        type={props.list ? undefined : props.type}
        maxLength={props.maxLength}
        id={props.id}
        name={props.name}
        placeholder={props.placeholder}
        defaultValue={props.value}
        disabled={props.disabled}
        list={props.list}
        onChange={props.onChange}
        onBlur={props.onBlur}
        onCopy={props.onCopy}
        onInput={props.onInput}
        required={props.required}
      />
    </>
  );
}

/**
 * Specifically renders a textarea
 * @param props
 * @returns
 */
function render_textarea(props: FormFieldProps) {
  return (
    <>
      <label className={STYLE_LABEL} htmlFor={props.id}>
        {props.label}
      </label>
      <textarea
        className="mb-0 h-auto w-full text-black bg-white border-[1px] border-black text-base disabled:bg-gray-400 rounded-none"
        name={props.name}
        id={props.id}
        rows={props.textarea ? props.textarea.rows : undefined}
        cols={props.textarea ? props.textarea.columns : undefined}
        defaultValue={props.value}
        disabled={props.disabled}
        onChange={props.onChange}
        onBlur={props.onBlur}
        onCopy={props.onCopy}
        onInput={props.onInput}
        required={props.required}
      ></textarea>
    </>
  );
}

/**
 * specifically renders a select element field
 * @param props
 * @returns
 */
function render_select(props: FormFieldProps) {
  return (
    <>
      <label className={STYLE_LABEL} htmlFor={props.id}>
        {props.label}
      </label>
      <select
        name={props.name}
        id={props.id}
        className="w-full bg-white border-black text-black border-[1px] disabled:bg-gray-400 h-10 rounded-none "
        value={props.value}
        disabled={props.disabled}
        onChange={props.onChange}
        onBlur={props.onBlur}
        onCopy={props.onCopy}
        onInput={props.onInput}
        required={props.required}
      >
        <option value="">--- Please Select ---</option>
        {(props.options || []).map((opt, index) => (
          <option key={props.id + '_select_option_' + index} value={opt.value}>
            {opt.text || opt.value}
          </option>
        ))}
      </select>
    </>
  );
}

/**
 * Specifically renders a checkbox
 * @param props
 * @returns
 */
function render_checkbox(props: FormFieldProps) {
  return (
    <>
      {props.type === 'toggle' ? (
        <>
          <label className={STYLE_LABEL + 'toggle-label'} htmlFor={props.id}>
            {props.label}
          </label>
          <br />
        </>
      ) : (
        <></>
      )}
      <label className={STYLE_LABEL} htmlFor={props.id}>
        <input
          className="mr-4"
          type="checkbox"
          name={props.name}
          id={props.id}
          defaultValue="yes"
          disabled={props.disabled}
          onChange={props.onChange}
          required={props.required}
        />
        {props.type === 'checkbox' ? (
          props.label
        ) : (
          <span className="slider"></span>
        )}
      </label>
    </>
  );
}

/**
 * Handles rendering a form field based on supplied properties
 * @param props
 * @returns
 */
function render_field(props: FormFieldProps) {
  switch (props.type) {
    case 'checkbox':
    case 'toggle':
      return render_checkbox(props);
    case 'select':
      return render_select(props);
    case 'textarea':
      return render_textarea(props);
    default:
      return render_default(props);
  }
}

/**
 * Display a form field with specific required/optional properties
 * @param props
 * @returns
 */
export const FormField = (props: FormFieldProps) => (
  <div className={'field mb-8 ' + (props.className || '')}>
    {render_field(props)}
  </div>
);

/**
 * Additional properties for form field groups.
 */
export interface FormFieldGroupProps extends React.PropsWithChildren {
  className?: string;
  label: string;
}

/**
 * Default styling/logic for handlign form field groups.  Should be used with Form Fields
 * @param props
 * @returns
 */
export const FormFieldGroup = (props: FormFieldGroupProps) => (
  <div className={'field-group ' + (props.className || '')}>
    <H3 className="flex-auto w-full my-8">{props.label}</H3>
    {props.children}
  </div>
);

/**
 * Default styling/logic for handling forms. Should be used with FormField and FormFieldGroup
 * @param props
 * @returns
 */
export const Form = (props: React.FormHTMLAttributes<HTMLFormElement>) => (
  <Container>
    <form {...props}>{props.children}</form>
  </Container>
);

export default Form;
