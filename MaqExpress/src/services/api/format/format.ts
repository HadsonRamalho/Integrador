export const formatDate = (dateparam: string | undefined, diff = -3) => {
    if (!dateparam) return "";
    console.log(dateparam);

    const date = new Date(dateparam);

    const brtOffset = diff * 60;
    const localDate = new Date(date.getTime() + brtOffset * 60 * 1000);

    const hours = String(localDate.getHours()).padStart(2, "0");
    const minutes = String(localDate.getMinutes()).padStart(2, "0");
    const day = String(localDate.getDate()).padStart(2, "0");
    const month = String(localDate.getMonth() + 1).padStart(2, "0");
    const year = localDate.getFullYear();

    return `${hours}:${minutes} ${day}/${month}/${year}`;
  };

export const formatCurrency = (value: number | bigint) => {
  return new Intl.NumberFormat("pt-BR", {
    style: "currency",
    currency: "BRL",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value);
};