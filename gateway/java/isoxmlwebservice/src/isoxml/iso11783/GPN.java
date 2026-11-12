//
// Diese Datei wurde mit der JavaTM Architecture for XML Binding(JAXB) Reference Implementation, v2.2.8-b130911.1802 generiert 
// Siehe <a href="http://java.sun.com/xml/jaxb">http://java.sun.com/xml/jaxb</a> 
// Änderungen an dieser Datei gehen bei einer Neukompilierung des Quellschemas verloren. 
// Generiert: 2021.12.09 um 11:30:16 AM CET 
//


package isoxml.iso11783;

import java.math.BigDecimal;
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
 *       &lt;choice maxOccurs="unbounded">
 *         &lt;element ref="{}LSG"/>
 *         &lt;element ref="{}PLN" minOccurs="0"/>
 *       &lt;/choice>
 *       &lt;attribute name="A" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}ID">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(GPN|GPN-)([0-9])+"/>
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
 *       &lt;attribute name="C" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *             &lt;enumeration value="4"/>
 *             &lt;enumeration value="5"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="D">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="E">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *             &lt;enumeration value="4"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="F">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *             &lt;enumeration value="4"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="G">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}decimal">
 *             &lt;minInclusive value="0.0"/>
 *             &lt;maxInclusive value="360.0"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="H">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedLong">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="4294967294"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="I">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}NMTOKEN">
 *             &lt;enumeration value="0"/>
 *             &lt;enumeration value="1"/>
 *             &lt;enumeration value="2"/>
 *             &lt;enumeration value="3"/>
 *             &lt;enumeration value="4"/>
 *             &lt;enumeration value="5"/>
 *             &lt;enumeration value="6"/>
 *             &lt;enumeration value="7"/>
 *             &lt;enumeration value="8"/>
 *             &lt;enumeration value="16"/>
 *             &lt;enumeration value="17"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="J">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}decimal">
 *             &lt;minInclusive value="0.0"/>
 *             &lt;maxInclusive value="65.0"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="K">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}decimal">
 *             &lt;minInclusive value="0.0"/>
 *             &lt;maxInclusive value="65.0"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="L">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}IDREF">
 *             &lt;minLength value="4"/>
 *             &lt;maxLength value="14"/>
 *             &lt;pattern value="(BSN|BSN-)([0-9])+"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="M">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="N">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedLong">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="4294967294"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="O">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedLong">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="4294967294"/>
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
    "lsgOrPLN"
})
@XmlRootElement(name = "GPN")
public class GPN {

    @XmlElements({
        @XmlElement(name = "LSG", type = LSG.class),
        @XmlElement(name = "PLN", type = PLN.class)
    })
    protected List<Object> lsgOrPLN;
    @XmlAttribute(name = "A", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    @XmlID
    protected String a;
    @XmlAttribute(name = "B")
    protected String b;
    @XmlAttribute(name = "C", required = true)
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String c;
    @XmlAttribute(name = "D")
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String d;
    @XmlAttribute(name = "E")
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String e;
    @XmlAttribute(name = "F")
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String f;
    @XmlAttribute(name = "G")
    protected BigDecimal g;
    @XmlAttribute(name = "H")
    protected Long h;
    @XmlAttribute(name = "I")
    @XmlJavaTypeAdapter(CollapsedStringAdapter.class)
    protected String i;
    @XmlAttribute(name = "J")
    protected BigDecimal j;
    @XmlAttribute(name = "K")
    protected BigDecimal k;
    @XmlAttribute(name = "L")
    @XmlIDREF
    protected Object l;
    @XmlAttribute(name = "M")
    protected String m;
    @XmlAttribute(name = "N")
    protected Long n;
    @XmlAttribute(name = "O")
    protected Long o;

    /**
     * Gets the value of the lsgOrPLN property.
     * 
     * <p>
     * This accessor method returns a reference to the live list,
     * not a snapshot. Therefore any modification you make to the
     * returned list will be present inside the JAXB object.
     * This is why there is not a <CODE>set</CODE> method for the lsgOrPLN property.
     * 
     * <p>
     * For example, to add a new item, do as follows:
     * <pre>
     *    getLSGOrPLN().add(newItem);
     * </pre>
     * 
     * 
     * <p>
     * Objects of the following type(s) are allowed in the list
     * {@link LSG }
     * {@link PLN }
     * 
     * 
     */
    public List<Object> getLSGOrPLN() {
        if (lsgOrPLN == null) {
            lsgOrPLN = new ArrayList<Object>();
        }
        return this.lsgOrPLN;
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
     *     {@link String }
     *     
     */
    public String getC() {
        return c;
    }

    /**
     * Legt den Wert der c-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setC(String value) {
        this.c = value;
    }

    /**
     * Ruft den Wert der d-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getD() {
        return d;
    }

    /**
     * Legt den Wert der d-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setD(String value) {
        this.d = value;
    }

    /**
     * Ruft den Wert der e-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getE() {
        return e;
    }

    /**
     * Legt den Wert der e-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setE(String value) {
        this.e = value;
    }

    /**
     * Ruft den Wert der f-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getF() {
        return f;
    }

    /**
     * Legt den Wert der f-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setF(String value) {
        this.f = value;
    }

    /**
     * Ruft den Wert der g-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link BigDecimal }
     *     
     */
    public BigDecimal getG() {
        return g;
    }

    /**
     * Legt den Wert der g-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link BigDecimal }
     *     
     */
    public void setG(BigDecimal value) {
        this.g = value;
    }

    /**
     * Ruft den Wert der h-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Long }
     *     
     */
    public Long getH() {
        return h;
    }

    /**
     * Legt den Wert der h-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Long }
     *     
     */
    public void setH(Long value) {
        this.h = value;
    }

    /**
     * Ruft den Wert der i-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getI() {
        return i;
    }

    /**
     * Legt den Wert der i-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setI(String value) {
        this.i = value;
    }

    /**
     * Ruft den Wert der j-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link BigDecimal }
     *     
     */
    public BigDecimal getJ() {
        return j;
    }

    /**
     * Legt den Wert der j-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link BigDecimal }
     *     
     */
    public void setJ(BigDecimal value) {
        this.j = value;
    }

    /**
     * Ruft den Wert der k-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link BigDecimal }
     *     
     */
    public BigDecimal getK() {
        return k;
    }

    /**
     * Legt den Wert der k-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link BigDecimal }
     *     
     */
    public void setK(BigDecimal value) {
        this.k = value;
    }

    /**
     * Ruft den Wert der l-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Object }
     *     
     */
    public Object getL() {
        return l;
    }

    /**
     * Legt den Wert der l-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Object }
     *     
     */
    public void setL(Object value) {
        this.l = value;
    }

    /**
     * Ruft den Wert der m-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getM() {
        return m;
    }

    /**
     * Legt den Wert der m-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setM(String value) {
        this.m = value;
    }

    /**
     * Ruft den Wert der n-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Long }
     *     
     */
    public Long getN() {
        return n;
    }

    /**
     * Legt den Wert der n-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Long }
     *     
     */
    public void setN(Long value) {
        this.n = value;
    }

    /**
     * Ruft den Wert der o-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Long }
     *     
     */
    public Long getO() {
        return o;
    }

    /**
     * Legt den Wert der o-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Long }
     *     
     */
    public void setO(Long value) {
        this.o = value;
    }

}
