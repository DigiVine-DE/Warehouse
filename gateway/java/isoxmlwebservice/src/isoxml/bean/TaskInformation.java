package isoxml.bean;

import java.util.ArrayList;

/**
 * Properties TaskData
 *
 */
public class TaskInformation {
    Long taskID = null;
    String taskName = null;
    String customerName = null;
    String farmName = null;
    String fruitGroupName = null;
    String fruitName = null;
    String plainName = null;
    Long plainSize = null;
    Integer effectiveTotalDistance = null;
    Integer effectiveTotalTime = null;
    Integer totalArea = null;
    //harvest
    Integer yieldTotalMass = null;
    Integer averageTartaricAcid = null;
    Integer averageMalicAcid = null;
    Integer averageGlucose = null;
    Integer averageFructose = null;
    //spray
    ArrayList<TaskInformationPlantProtection> plantProtection = null; 
    Integer applicationTotalVolume = null;
    //geometry
    String geometryType = null;
    ArrayList <TaskInformationPoint> coordinates = null;
    String versionMajor = null;
    String versionMinor = null;
    String managementSoftwareManufacturer = null;
    String managementSoftwareVersion = null;
    String dataTransferOrigin = null;
    
    
    
    public Long getTaskID() {
        return taskID;
    }
    public void setTaskID(Long taskID) {
        this.taskID = taskID;
    }
    public String getVersionMajor() {
        return versionMajor;
    }
    public void setVersionMajor(String versionMajor) {
        this.versionMajor = versionMajor;
    }
    public String getVersionMinor() {
        return versionMinor;
    }
    public void setVersionMinor(String versionMinor) {
        this.versionMinor = versionMinor;
    }
    public String getManagementSoftwareVersion() {
        return managementSoftwareVersion;
    }
    public void setManagementSoftwareVersion(String managementSoftwareVersion) {
        this.managementSoftwareVersion = managementSoftwareVersion;
    }
    public String getDataTransferOrigin() {
        return dataTransferOrigin;
    }
    public void setDataTransferOrigin(String dataTransferOrigin) {
        this.dataTransferOrigin = dataTransferOrigin;
    }
    public String getTaskName() {
        return taskName;
    }
    public void setTaskName(String taskName) {
        this.taskName = taskName;
    }
    public String getCustomerName() {
        return customerName;
    }
    public void setCustomerName(String customerName) {
        this.customerName = customerName;
    }
    public String getFarmName() {
        return farmName;
    }
    public void setFarmName(String farmName) {
        this.farmName = farmName;
    }
    public String getFruitGroupName() {
        return fruitGroupName;
    }
    public void setFruitGroupName(String fruitGroupName) {
        this.fruitGroupName = fruitGroupName;
    }
    public String getFruitName() {
        return fruitName;
    }
    public void setFruitName(String fruitName) {
        this.fruitName = fruitName;
    }
    public String getPlainName() {
        return plainName;
    }
    public void setPlainName(String plainName) {
        this.plainName = plainName;
    }
    public Integer getYieldTotalMass() {
        return yieldTotalMass;
    }
    public void setYieldTotalMass(Integer yieldTotalMass) {
        this.yieldTotalMass = yieldTotalMass;
    }
    public Integer getTotalArea() {
        return totalArea;
    }
    public void setTotalArea(Integer totalArea) {
        this.totalArea = totalArea;
    }
    public Integer getEffectiveTotalDistance() {
        return effectiveTotalDistance;
    }
    public void setEffectiveTotalDistance(Integer effectiveTotalDistance) {
        this.effectiveTotalDistance = effectiveTotalDistance;
    }
    public Integer getEffectiveTotalTime() {
        return effectiveTotalTime;
    }
    public void setEffectiveTotalTime(Integer effectiveTotalTime) {
        this.effectiveTotalTime = effectiveTotalTime;
    }
    public Integer getAverageTartaricAcid() {
        return averageTartaricAcid;
    }
    public void setAverageTartaricAcid(Integer averageTartaricAcid) {
        this.averageTartaricAcid = averageTartaricAcid;
    }
    public Integer getAverageMalicAcid() {
        return averageMalicAcid;
    }
    public void setAverageMalicAcid(Integer averageMalicAcid) {
        this.averageMalicAcid = averageMalicAcid;
    }
    public Integer getAverageGlucose() {
        return averageGlucose;
    }
    public void setAverageGlucose(Integer averageGlucose) {
        this.averageGlucose = averageGlucose;
    }
    public Integer getAverageFructose() {
        return averageFructose;
    }
    public void setAverageFructose(Integer averageFructose) {
        this.averageFructose = averageFructose;
    }
    
    public ArrayList<TaskInformationPlantProtection> getPlantProtection() {
        return plantProtection;
    }
    public void setPlantProtection(ArrayList<TaskInformationPlantProtection> plantProtection) {
        this.plantProtection = plantProtection;
    }
    public Integer getApplicationTotalVolume() {
        return applicationTotalVolume;
    }
    public void setApplicationTotalVolume(Integer applicationTotalVolume) {
        this.applicationTotalVolume = applicationTotalVolume;
    }
    public String getGeometryType() {
        return geometryType;
    }
    public void setGeometryType(String geometryType) {
        this.geometryType = geometryType;
    }
    public ArrayList<TaskInformationPoint> getCoordinates() {
        return coordinates;
    }
    public void setCoordinates(ArrayList<TaskInformationPoint> coordinates) {
        this.coordinates = coordinates;
    }
    public void addPointToCoordinates(TaskInformationPoint point) {
        if(this.coordinates == null) {
            this.coordinates = new ArrayList<TaskInformationPoint>();
        }
        this.coordinates.add(point);
    }
    public void addResourceToPlantProtection(TaskInformationPlantProtection resource) {
        if(this.plantProtection == null) {
            this.plantProtection = new ArrayList<TaskInformationPlantProtection>();
        }
        this.plantProtection.add(resource);
    }
    public String getManagementSoftwareManufacturer() {
        return managementSoftwareManufacturer;
    }
    public void setManagementSoftwareManufacturer(String managementSoftwareManufacturer) {
        this.managementSoftwareManufacturer = managementSoftwareManufacturer;
    }
    public Long getPlainSize() {
        return plainSize;
    }
    public void setPlainSize(Long plainSize) {
        this.plainSize = plainSize;
    }
}
