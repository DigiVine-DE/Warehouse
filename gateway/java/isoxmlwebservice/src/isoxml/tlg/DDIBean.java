package isoxml.tlg;

import org.apache.log4j.Logger;

/**
 * bean for ddi information
 *
 */
public class DDIBean {
  private static Logger ddiBeanLogger = Logger.getLogger(DDIBean.class.getName());    
    
  private String ddIdentifier = null; 
  private String ddeName = null; 
  private String definition = null; 
  private String comment = null; 
  private String deviceClassses = null; 
  private String unitSymbol = null; 
  private String unitQuantity = null; 
  private String bitResolution = null; 
  private String saespn = null; 
  private String minimumValue = null; 
  private String maximumValue = null; 
  private String submitBy = null; 
  private String submitDate = null; 
  private String submitCompany = null; 
  private String status = null; 
  private String statusDate = null; 
  private String requestDocument = null; 
  private String annexReleaseDate = null; 
  private String statusCommentsInteger = null;
  
  public DDIBean() {
  }

  public String getDdIdentifier () {
    return ddIdentifier;
  }

  public void setDdIdentifier (String ddIdentifier) {
    this.ddIdentifier = ddIdentifier;
  }

  public String getDdeName () {
    return ddeName;
  }

  public void setDdeName (String ddeName) {
    this.ddeName = ddeName;
  }

  public String getDefinition () {
    return definition;
  }

  public void setDefinition (String definition) {
    this.definition = definition;
  }

  public String getComment () {
    return comment;
  }

  public void setComment (String comment) {
    this.comment = comment;
  }

  public String getDeviceClassses () {
    return deviceClassses;
  }

  public void setDeviceClassses (String deviceClassses) {
    this.deviceClassses = deviceClassses;
  }

  public String getUnitSymbol () {
    return unitSymbol;
  }

  public void setUnitSymbol (String unitSymbol) {
    this.unitSymbol = unitSymbol;
  }

  public String getUnitQuantity () {
    return unitQuantity;
  }

  public void setUnitQuantity (String unitQuantity) {
    this.unitQuantity = unitQuantity;
  }

  public String getBitResolution () {
    return bitResolution;
  }

  public void setBitResolution (String bitResolution) {
    this.bitResolution = bitResolution;
  }
  
  public String getSaespn () {
    return saespn;
  }

  public void setSaespn (String saespn) {
    this.saespn = saespn;
  }

  public String getMinimumValue () {
    return minimumValue;
  }

  public void setMinimumValue (String minimumValue) {
    this.minimumValue = minimumValue;
  }

  public String getMaximumValue () {
    return maximumValue;
  }

  public void setMaximumValue (String maximumValue) {
    this.maximumValue = maximumValue;
  }

  public String getSubmitBy () {
    return submitBy;
  }

  public void setSubmitBy (String submitBy) {
    this.submitBy = submitBy;
  }

  public String getSubmitDate () {
    return submitDate;
  }

  public void setSubmitDate (String submitDate) {
    this.submitDate = submitDate;
  }

  public String getSubmitCompany () {
    return submitCompany;
  }

  public void setSubmitCompany (String submitCompany) {
    this.submitCompany = submitCompany;
  }

  public String getStatus () {
    return status;
  }

  public void setStatus (String status) {
    this.status = status;
  }

  public String getStatusDate () {
    return statusDate;
  }

  public void setStatusDate (String statusDate) {
    this.statusDate = statusDate;
  }

  public String getRequestDocument () {
    return requestDocument;
  }

  public void setRequestDocument (String requestDocument) {
    this.requestDocument = requestDocument;
  }

  public String getAnnexReleaseDate () {
    return annexReleaseDate;
  }

  public void setAnnexReleaseDate (String annexReleaseDate) {
    this.annexReleaseDate = annexReleaseDate;
  }

  public String getStatusCommentsInteger () {
    return statusCommentsInteger;
  }

  public void setStatusCommentsInteger (String statusCommentsInteger) {
    this.statusCommentsInteger = statusCommentsInteger;
  }
 
  public void debug(){
    ddiBeanLogger.info("debugAusgabe:" 
        + "ddIdentifier."+this.ddIdentifier
        +",ddeName."+this.ddeName
        +",definition."+this.definition
        +",comment."+this.comment
        +",deviceClassses."+this.deviceClassses
        +",unitSymbol."+this.unitSymbol
        +",unitQuantity."+this.unitQuantity
        +",bitResolution."+this.bitResolution
        +",saespn."+this.saespn
        +",minimumValue."+this.minimumValue
        +",maximumValue."+this.maximumValue
        +",submitBy."+this.submitBy
        +",submitDate."+this.submitDate
        +",submitCompany."+this.submitCompany
        +",status."+this.status
        +",statusDate."+this.statusDate
        +",requestDocument."+this.requestDocument
        +",annexReleaseDate."+this.annexReleaseDate
        +",statusCommentsInteger."+this.statusCommentsInteger);
  }
}
