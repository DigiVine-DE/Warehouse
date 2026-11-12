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
import jakarta.xml.bind.annotation.XmlID;
import jakarta.xml.bind.annotation.XmlIDREF;
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
 *         &lt;element ref="{}TZN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}TIM" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}OTP" minOccurs="0"/>
 *         &lt;element ref="{}WAN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}DAN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CNN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}PAN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}DLT" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}CAN" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}TLG" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}GRD" minOccurs="0"/>
 *         &lt;element ref="{}CAT" maxOccurs="unbounded" minOccurs="0"/>
 *         &lt;element ref="{}GAN" maxOccurs="unbounded" minOccurs="0"/>
 *       &lt;/choice>
 *       &lt;attribute name="A" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}ID">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(TSK|TSK-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="B">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="C">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}IDREF">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(CTR|CTR-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="D">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}IDREF">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(FRM|FRM-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="E">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}IDREF">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(PFD|PFD-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="F">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}IDREF">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(WKR|WKR-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="G" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *             &lt;enumeration value="4"/>
 *             &lt;enumeration value="5"/>
 *             &lt;enumeration value="6"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="H">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedByte">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="254"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="I">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedByte">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="254"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="J">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedByte">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="254"/>
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
    "tznOrTIMOrOTP"
})
@XmlRootElement(name = "TSK")
public class TSK {

    @XmlElements({
        @XmlElement(name = "TZN", type = TZN.class),
        @XmlElement(name = "TIM", type = TIM.class),
        @XmlElement(name = "OTP", type = OTP.class),
        @XmlElement(name = "WAN", type = WAN.class),
        @XmlElement(name = "DAN", type = DAN.class),
        @XmlElement(name = "CNN", type = CNN.class),
        @XmlElement(name = "PAN", type = PAN.class),
        @XmlElement(name = "DLT", type = DLT.class),
        @XmlElement(name = "CAN", type = CAN.class),
        @XmlElement(name = "TLG", type = TLG.class),
        @XmlElement(name = "GRD", type = GRD.class),
        @XmlElement(name = "CAT", type = CAT.class),
        @XmlElement(name = "GAN", type = GAN.class)
    })
    protected List<Object> tznOrTIMOrOTP;
    @XmlAttribute(name = "A", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    @XmlID
    protected String a;
    @XmlAttribute(name = "B")
    protected String b;
    @XmlAttribute(name = "C")
    @XmlIDREF
    protected Object c;
    @XmlAttribute(name = "D")
    @XmlIDREF
    protected Object d;
    @XmlAttribute(name = "E")
    @XmlIDREF
    protected Object e;
    @XmlAttribute(name = "F")
    @XmlIDREF
    protected Object f;
    @XmlAttribute(name = "G", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String g;
    @XmlAttribute(name = "H")
    protected Short h;
    @XmlAttribute(name = "I")
    protected Short i;
    @XmlAttribute(name = "J")
    protected Short j;

    /**
     * Gets the value of the tznOrTIMOrOTP property.
     * 
     * <p>
     * This accessor method returns a reference to the live list,
     * not a snapshot. Therefore any modification you make to the
     * returned list will be present inside the JAXB object.
     * This is why there is not a <CODE>set</CODE> method for the tznOrTIMOrOTP property.
     * 
     * <p>
     * For example, to add a new item, do as follows:
     * <pre>
     *    getTZNOrTIMOrOTP().add(newItem);
     * </pre>
     * 
     * 
     * <p>
     * Objects of the following type(s) are allowed in the list
     * {@link TZN }
     * {@link TIM }
     * {@link OTP }
     * {@link WAN }
     * {@link DAN }
     * {@link CNN }
     * {@link PAN }
     * {@link DLT }
     * {@link CAN }
     * {@link TLG }
     * {@link GRD }
     * {@link CAT }
     * {@link GAN }
     * 
     * 
     */
    public List<Object> getTZNOrTIMOrOTP() {
        if (tznOrTIMOrOTP == null) {
            tznOrTIMOrOTP = new ArrayList<Object>();
        }
        return this.tznOrTIMOrOTP;
    }

    /**
     * Ruft den Wert der a-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getA() {
        return a;
    }

    /**
     * Legt den Wert der a-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setA(String value) {
        this.a = value;
    }

    /**
     * Ruft den Wert der b-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getB() {
        return b;
    }

    /**
     * Legt den Wert der b-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setB(String value) {
        this.b = value;
    }

    /**
     * Ruft den Wert der c-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Object }
     *     
     */
    public Object getC() {
        return c;
    }

    /**
     * Legt den Wert der c-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Object }
     *     
     */
    public void setC(Object value) {
        this.c = value;
    }

    /**
     * Ruft den Wert der d-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Object }
     *     
     */
    public Object getD() {
        return d;
    }

    /**
     * Legt den Wert der d-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Object }
     *     
     */
    public void setD(Object value) {
        this.d = value;
    }

    /**
     * Ruft den Wert der e-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Object }
     *     
     */
    public Object getE() {
        return e;
    }

    /**
     * Legt den Wert der e-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Object }
     *     
     */
    public void setE(Object value) {
        this.e = value;
    }

    /**
     * Ruft den Wert der f-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Object }
     *     
     */
    public Object getF() {
        return f;
    }

    /**
     * Legt den Wert der f-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Object }
     *     
     */
    public void setF(Object value) {
        this.f = value;
    }

    /**
     * Ruft den Wert der g-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getG() {
        return g;
    }

    /**
     * Legt den Wert der g-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setG(String value) {
        this.g = value;
    }

    /**
     * Ruft den Wert der h-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Short }
     *     
     */
    public Short getH() {
        return h;
    }

    /**
     * Legt den Wert der h-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Short }
     *     
     */
    public void setH(Short value) {
        this.h = value;
    }

    /**
     * Ruft den Wert der i-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Short }
     *     
     */
    public Short getI() {
        return i;
    }

    /**
     * Legt den Wert der i-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Short }
     *     
     */
    public void setI(Short value) {
        this.i = value;
    }

    /**
     * Ruft den Wert der j-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Short }
     *     
     */
    public Short getJ() {
        return j;
    }

    /**
     * Legt den Wert der j-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Short }
     *     
     */
    public void setJ(Short value) {
        this.j = value;
    }

}
