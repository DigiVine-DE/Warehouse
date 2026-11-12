package isoxml.bean;


/**
 * Plant Protection TaskData
 *
 */
public class TaskInformationPlantProtection {
    private String name = null;
    private Integer amount = null;
    private String unit = null;
    public String getName() {
        return name;
    }
    public void setName(String name) {
        this.name = name;
    }
   
    public Integer getAmount() {
        return amount;
    }

    public void setAmount(Integer amount) {
        this.amount = amount;
    }
    public String getUnit() {
        return unit;
    }
    public void setUnit(String unit) {
        this.unit = unit;
    }
}
