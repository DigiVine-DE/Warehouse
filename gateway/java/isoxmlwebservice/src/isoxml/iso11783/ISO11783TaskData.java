//
// Diese Datei wurde mit der JavaTM Architecture for XML Binding(JAXB) Reference Implementation, v2.2.8-b130911.1802 generiert 
// Siehe <a href="http://java.sun.com/xml/jaxb">http://java.sun.com/xml/jaxb</a> 
// Änderungen an dieser Datei gehen bei einer Neukompilierung des Quellschemas verloren. 
// Generiert: 2021.12.09 um 11:30:16 AM CET 
//


package isoxml.iso11783;

import java.util.ArrayList;
import java.util.List;

import jakarta.xml.bind.annotation.XmlAccessType;
import jakarta.xml.bind.annotation.XmlAccessorType;
import jakarta.xml.bind.annotation.XmlAttribute;
import jakarta.xml.bind.annotation.XmlElement;
import jakarta.xml.bind.annotation.XmlElements;
import jakarta.xml.bind.annotation.XmlRootElement;
import jakarta.xml.bind.annotation.XmlType;
import jakarta.xml.bind.annotation.adapters.CollapsedStringAdapter;
import jakarta.xml.bind.annotation.adapters.XmlJavaTypeAdapter;


/**
 * <p>Java-Klasse für anonymous complex type.
 * 
 * <p>Das folgende Schemafragment gibt den erwarteten Content an, der in dieser Klasse enthalten ist.
 * 
 * <pre>
 * &lt;complexType>
 *   &lt;complexContent>
 *     &lt;restriction base="{http://www.w3.org/2001/XMLSchema}anyType">
 *       &lt;choice maxOccurs="unbounded" minOccurs="0">
 *         &lt;element ref="{}AFE" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}BSN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CCT" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CCG" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CLD" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CTP" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CPC" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CTR" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}DVC" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}FRM" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}OTQ" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}PFD" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}PDT" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}PGP" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}TSK" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}TCC" minOccurs="0"/>
 *         &lt;element ref="{}VPN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}WKR" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}XFR" maxOccurs="unbounded" minOccurs="0"/>
 *       &lt;/choice>
 *       &lt;attribute name="VersionMajor" use="required" fixed="4">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="4"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="VersionMinor" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="0"/>
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="ManagementSoftwareManufacturer" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="ManagementSoftwareVersion" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="TaskControllerManufacturer">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="TaskControllerVersion">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="DataTransferOrigin" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="lang">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}language">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *     &lt;/restriction>
 *   &lt;/complexContent>
 * &lt;/complexType>
 * </pre>
 * 
 * 
 */
@XmlAccessorType(XmlAccessType.FIELD)
@XmlType(name = "", propOrder = {
    "xmlElements"
})
@XmlRootElement(name = "ISO11783_TaskData")
public class ISO11783TaskData {

    @XmlElements({
        @XmlElement(name = "AFE", type = AFE.class),
        @XmlElement(name = "BSN", type = BSN.class),
        @XmlElement(name = "CCT", type = CCT.class),
        @XmlElement(name = "CCG", type = CCG.class),
        @XmlElement(name = "CLD", type = CLD.class),
        @XmlElement(name = "CTP", type = CTP.class),
        @XmlElement(name = "CPC", type = CPC.class),
        @XmlElement(name = "CTR", type = CTR.class),
        @XmlElement(name = "DVC", type = DVC.class),
        @XmlElement(name = "FRM", type = FRM.class),
        @XmlElement(name = "OTQ", type = OTQ.class),
        @XmlElement(name = "PFD", type = PFD.class),
        @XmlElement(name = "PDT", type = PDT.class),
        @XmlElement(name = "PGP", type = PGP.class),
        @XmlElement(name = "TSK", type = TSK.class),
        @XmlElement(name = "TCC", type = TCC.class),
        @XmlElement(name = "VPN", type = VPN.class),
        @XmlElement(name = "WKR", type = WKR.class),
        @XmlElement(name = "XFR", type = XFR.class)
    })
    protected List<Object> xmlElements;
    @XmlAttribute(name = "VersionMajor", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String versionMajor;
    @XmlAttribute(name = "VersionMinor", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String versionMinor;
    @XmlAttribute(name = "ManagementSoftwareManufacturer", required = true)
    protected String managementSoftwareManufacturer;
    @XmlAttribute(name = "ManagementSoftwareVersion", required = true)
    protected String managementSoftwareVersion;
    @XmlAttribute(name = "TaskControllerManufacturer")
    protected String taskControllerManufacturer;
    @XmlAttribute(name = "TaskControllerVersion")
    protected String taskControllerVersion;
    @XmlAttribute(name = "DataTransferOrigin", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String dataTransferOrigin;
    @XmlAttribute(name = "lang")
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String lang;

    /**
     * Gets the value of the afeOrBSNOrCCT property.
     * 
     * <p>
     * This accessor method returns a reference to the live list,
     * not a snapshot. Therefore any modification you make to the
     * returned list will be present inside the JAXB object.
     * This is why there is not a <CODE>set</CODE> method for the afeOrBSNOrCCT property.
     * 
     * <p>
     * For example, to add a new item, do as follows:
     * <pre>
     *    getAFEOrBSNOrCCT().add(newItem);
     * </pre>
     * 
     * 
     * <p>
     * Objects of the following type(s) are allowed in the list
     * {@link AFE }
     * {@link BSN }
     * {@link CCT }
     * {@link CCG }
     * {@link CLD }
     * {@link CTP }
     * {@link CPC }
     * {@link CTR }
     * {@link DVC }
     * {@link FRM }
     * {@link OTQ }
     * {@link PFD }
     * {@link PDT }
     * {@link PGP }
     * {@link TSK }
     * {@link TCC }
     * {@link VPN }
     * {@link WKR }
     * {@link XFR }
     * 
     * 
     */
    public List<Object> getXmlElements() {
        if (xmlElements == null) {
          xmlElements = new ArrayList<Object>();
        }
        return this.xmlElements;
    }
    
    /**
     * Ruft den Wert der versionMajor-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getVersionMajor() {
        if (versionMajor == null) {
            return "4";
        } else {
            return versionMajor;
        }
    }

    /**
     * Legt den Wert der versionMajor-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setVersionMajor(String value) {
        this.versionMajor = value;
    }

    /**
     * Ruft den Wert der versionMinor-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getVersionMinor() {
        return versionMinor;
    }

    /**
     * Legt den Wert der versionMinor-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setVersionMinor(String value) {
        this.versionMinor = value;
    }

    /**
     * Ruft den Wert der managementSoftwareManufacturer-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getManagementSoftwareManufacturer() {
        return managementSoftwareManufacturer;
    }

    /**
     * Legt den Wert der managementSoftwareManufacturer-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setManagementSoftwareManufacturer(String value) {
        this.managementSoftwareManufacturer = value;
    }

    /**
     * Ruft den Wert der managementSoftwareVersion-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getManagementSoftwareVersion() {
        return managementSoftwareVersion;
    }

    /**
     * Legt den Wert der managementSoftwareVersion-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setManagementSoftwareVersion(String value) {
        this.managementSoftwareVersion = value;
    }

    /**
     * Ruft den Wert der taskControllerManufacturer-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getTaskControllerManufacturer() {
        return taskControllerManufacturer;
    }

    /**
     * Legt den Wert der taskControllerManufacturer-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setTaskControllerManufacturer(String value) {
        this.taskControllerManufacturer = value;
    }

    /**
     * Ruft den Wert der taskControllerVersion-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getTaskControllerVersion() {
        return taskControllerVersion;
    }

    /**
     * Legt den Wert der taskControllerVersion-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setTaskControllerVersion(String value) {
        this.taskControllerVersion = value;
    }

    /**
     * Ruft den Wert der dataTransferOrigin-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getDataTransferOrigin() {
        return dataTransferOrigin;
    }

    /**
     * Legt den Wert der dataTransferOrigin-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setDataTransferOrigin(String value) {
        this.dataTransferOrigin = value;
    }

    /**
     * Ruft den Wert der lang-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getLang() {
        return lang;
    }

    /**
     * Legt den Wert der lang-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setLang(String value) {
        this.lang = value;
    }

}
